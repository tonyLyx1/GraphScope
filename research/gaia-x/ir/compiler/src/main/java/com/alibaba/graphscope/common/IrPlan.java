/*
 * Copyright 2020 Alibaba Group Holding Limited.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.alibaba.graphscope.common;

import com.alibaba.graphscope.common.exception.AppendInterOpException;
import com.alibaba.graphscope.common.exception.BuildPhysicalException;
import com.alibaba.graphscope.common.exception.InterOpIllegalArgException;
import com.alibaba.graphscope.common.exception.InterOpUnsupportedException;
import com.alibaba.graphscope.common.intermediate.operator.*;
import com.alibaba.graphscope.common.jna.IrCoreLibrary;
import com.alibaba.graphscope.common.jna.type.*;
import com.sun.jna.Pointer;
import com.sun.jna.ptr.IntByReference;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.Closeable;
import java.util.List;
import java.util.Optional;
import java.util.function.Function;

// represent ir plan as a chain of operators
public class IrPlan implements Closeable {
    private static IrCoreLibrary irCoreLib = IrCoreLibrary.INSTANCE;
    private static Logger logger = LoggerFactory.getLogger(IrPlan.class);
    private Pointer ptrPlan;
    private IntByReference oprIdx;

    // call libc to transform from InterOpBase to c structure
    private enum TransformFactory implements Function<InterOpBase, Pointer> {
        SCAN_FUSION_OP {
            @Override
            public Pointer apply(InterOpBase baseOp) {
                ScanFusionOp op = (ScanFusionOp) baseOp;
                Optional<OpArg> scanOpt = op.getScanOpt();
                if (!scanOpt.isPresent()) {
                    throw new InterOpIllegalArgException(baseOp.getClass(), "scanOpt", "not present");
                }
                FfiScanOpt ffiScanOpt = (FfiScanOpt) scanOpt.get().getArg();
                Pointer scan = irCoreLib.initScanOperator(ffiScanOpt);
                Optional<OpArg> labels = op.getLabels();
                if (labels.isPresent()) {
                    List<FfiNameOrId.ByValue> ffiLabels = (List<FfiNameOrId.ByValue>) labels.get().getArg();
                    for (FfiNameOrId.ByValue label : ffiLabels) {
                        ResultCode resultCode = irCoreLib.addScanTableName(scan, label);
                        if (resultCode != ResultCode.Success) {
                            throw new InterOpIllegalArgException(baseOp.getClass(),
                                    "labels", "addScanTableName returns " + resultCode.name());
                        }
                    }
                }
                Optional<OpArg> predicate = op.getPredicate();
                if (predicate.isPresent()) {
                    String expr = (String) predicate.get().getArg();
                    ResultCode resultCode = irCoreLib.setScanPredicate(scan, expr);
                    if (resultCode != ResultCode.Success) {
                        throw new InterOpIllegalArgException(baseOp.getClass(),
                                "predicate", "setScanPredicate returns " + resultCode.name());
                    }
                }
                // todo: add predicates
                // todo: add limit
                Optional<OpArg> ids = op.getIds();
                if (ids.isPresent()) {
                    Pointer pairs = irCoreLib.initKvEquivPairs();
                    List<FfiConst.ByValue> ffiIds = (List<FfiConst.ByValue>) ids.get().getArg();
                    scan = irCoreLib.initIdxscanOperator(scan);
                    for (int i = 0; i < ffiIds.size(); ++i) {
                        if (i == 1) {
                            logger.error("{}", "indexing based query can only support one argument, ignore others");
                            break;
                        }
                        ResultCode resultCode = irCoreLib.andKvEquivPair(pairs, irCoreLib.asIdKey(), ffiIds.get(i));
                        if (resultCode != ResultCode.Success) {
                            throw new InterOpIllegalArgException(baseOp.getClass(),
                                    "ids", "andKvEquivPair returns " + resultCode.name());
                        }
                        resultCode = irCoreLib.addIdxscanKvEquivPairs(scan, pairs);
                        if (resultCode != ResultCode.Success) {
                            throw new InterOpIllegalArgException(baseOp.getClass(),
                                    "ids", "addIdxscanKvEquivPairs returns " + resultCode.name());
                        }
                    }
                }
                return scan;
            }
        },
        SELECT_OP {
            @Override
            public Pointer apply(InterOpBase baseOp) {
                SelectOp op = (SelectOp) baseOp;
                Optional<OpArg> predicate = op.getPredicate();
                if (!predicate.isPresent()) {
                    throw new InterOpIllegalArgException(baseOp.getClass(), "predicate", "not present");
                }
                String expr = (String) predicate.get().getArg();
                Pointer select = irCoreLib.initSelectOperator();
                ResultCode resultCode = irCoreLib.setSelectPredicate(select, expr);
                if (resultCode != ResultCode.Success) {
                    throw new InterOpIllegalArgException(baseOp.getClass(),
                            "predicate", "setSelectPredicate returns " + resultCode.name());
                }
                return select;
            }
        },
        EXPAND_OP {
            @Override
            public Pointer apply(InterOpBase baseOp) {
                ExpandOp op = (ExpandOp) baseOp;
                Optional<OpArg> direction = op.getDirection();
                if (!direction.isPresent()) {
                    throw new InterOpIllegalArgException(baseOp.getClass(), "direction", "not present");
                }
                FfiDirection ffiDirection = (FfiDirection) direction.get().getArg();
                Pointer expand = irCoreLib.initExpandBase(ffiDirection);
                Optional<OpArg> labels = op.getLabels();
                if (labels.isPresent()) {
                    List<FfiNameOrId.ByValue> ffiLabels = (List<FfiNameOrId.ByValue>) labels.get().getArg();
                    for (FfiNameOrId.ByValue label : ffiLabels) {
                        ResultCode resultCode = irCoreLib.addExpandLabel(expand, label);
                        if (resultCode != ResultCode.Success) {
                            throw new InterOpIllegalArgException(baseOp.getClass(),
                                    "labels", "addExpandLabel returns " + resultCode.name());
                        }
                    }
                }
                // todo: add properties
                // todo: add predicates
                // todo: add limit
                Optional<OpArg> edgeOpt = op.getEdgeOpt();
                if (!edgeOpt.isPresent()) {
                    throw new InterOpIllegalArgException(baseOp.getClass(), "edgeOpt", "not present");
                }
                Boolean isEdge = (Boolean) edgeOpt.get().getArg();
                expand = irCoreLib.initEdgexpdOperator(expand, isEdge);
                return expand;
            }
        },
        LIMIT_OP {
            @Override
            public Pointer apply(InterOpBase baseOp) {
                LimitOp op = (LimitOp) baseOp;
                Optional<OpArg> lower = op.getLower();
                if (!lower.isPresent()) {
                    throw new InterOpIllegalArgException(baseOp.getClass(), "lower", "not present");
                }
                Optional<OpArg> upper = op.getUpper();
                if (!upper.isPresent()) {
                    throw new InterOpIllegalArgException(baseOp.getClass(), "upper", "not present");
                }
                Pointer ptrLimit = irCoreLib.initLimitOperator();
                ResultCode resultCode = irCoreLib.setLimitRange(ptrLimit,
                        (Integer) lower.get().getArg(), (Integer) upper.get().getArg());
                if (resultCode != ResultCode.Success) {
                    throw new InterOpIllegalArgException(baseOp.getClass(),
                            "lower+upper", "setLimitRange returns " + resultCode.name());
                }
                return ptrLimit;
            }
        }
    }

    public IrPlan() {
        this.ptrPlan = irCoreLib.initLogicalPlan();
        this.oprIdx = new IntByReference(0);
    }

    @Override
    public void close() {
        if (ptrPlan != null) {
            irCoreLib.destroyLogicalPlan(ptrPlan);
        }
    }

    public void debug() {
        if (ptrPlan != null) {
            irCoreLib.debugPlan(ptrPlan);
        }
    }

    public void appendInterOp(InterOpBase base) throws
            InterOpIllegalArgException, InterOpUnsupportedException, AppendInterOpException {
        ResultCode resultCode;
        if (base instanceof ScanFusionOp) {
            Pointer ptrScan = TransformFactory.SCAN_FUSION_OP.apply(base);
            if (((ScanFusionOp) base).getIds().isPresent()) {
                resultCode = irCoreLib.appendIdxscanOperator(ptrPlan, ptrScan, oprIdx.getValue(), oprIdx);
            } else {
                resultCode = irCoreLib.appendScanOperator(ptrPlan, ptrScan, oprIdx.getValue(), oprIdx);
            }
        } else if (base instanceof SelectOp) {
            Pointer ptrSelect = TransformFactory.SELECT_OP.apply(base);
            resultCode = irCoreLib.appendSelectOperator(ptrPlan, ptrSelect, oprIdx.getValue(), oprIdx);
        } else if (base instanceof ExpandOp) {
            Pointer ptrExpand = TransformFactory.EXPAND_OP.apply(base);
            resultCode = irCoreLib.appendEdgexpdOperator(ptrPlan, ptrExpand, oprIdx.getValue(), oprIdx);
        } else if (base instanceof LimitOp) {
            Pointer ptrLimit = TransformFactory.LIMIT_OP.apply(base);
            resultCode = irCoreLib.appendLimitOperator(ptrPlan, ptrLimit, oprIdx.getValue(), oprIdx);
        } else {
            throw new InterOpUnsupportedException(base.getClass(), "unimplemented yet");
        }
        if (resultCode != null && resultCode != ResultCode.Success) {
            throw new AppendInterOpException(base.getClass(), resultCode);
        }
    }

    public byte[] toPhysicalBytes() throws BuildPhysicalException {
        if (ptrPlan == null) {
            throw new BuildPhysicalException("ptrPlan is NullPointer");
        }
        FfiJobBuffer.ByValue buffer = irCoreLib.buildPhysicalPlan(ptrPlan);
        if (buffer.len == 0) {
            throw new BuildPhysicalException("call libc returns " + ResultCode.BuildJobError.name());
        }
        byte[] bytes = buffer.getBytes();
        buffer.close();
        return bytes;
    }
}
