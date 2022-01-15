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

package com.alibaba.graphscope.common.jna;

import com.alibaba.graphscope.common.jna.type.*;
import com.google.common.collect.ImmutableMap;
import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Pointer;
import com.sun.jna.ptr.IntByReference;

public interface IrCoreLibrary extends Library {
    IrCoreLibrary INSTANCE = Native.load("ir_core", IrCoreLibrary.class, ImmutableMap.of(
            Library.OPTION_TYPE_MAPPER, IrTypeMapper.INSTANCE,
            Library.OPTION_FUNCTION_MAPPER, IrFunctionMapper.INSTANCE));

    Pointer initLogicalPlan(boolean isPreprocess);

    void write_plan_to_json(Pointer plan, String jsonFile);

    void destroyLogicalPlan(Pointer plan);

    FfiJobBuffer.ByValue buildPhysicalPlan(Pointer plan, int workers, int servers);

    Pointer initScanOperator(FfiScanOpt opt);

    ResultCode setScanPredicate(Pointer scan, String predicate);

    ResultCode addScanTableName(Pointer scan, FfiNameOrId.ByValue tableName);

    ResultCode setScanAlias(Pointer scan, FfiAlias.ByValue alias);

    // set primary index
    Pointer initIndexPredicate();

    ResultCode andEquivPredicate(Pointer predicate, FfiProperty.ByValue key, FfiConst.ByValue value);

    ResultCode orEquivPredicate(Pointer predicate, FfiProperty.ByValue key, FfiConst.ByValue value);

    ResultCode addScanIndexPredicate(Pointer scan, Pointer predicate);

    ResultCode appendScanOperator(Pointer plan, Pointer scan, int parent, IntByReference oprIdx);

    Pointer initExpandBase(FfiDirection direction);

    ResultCode addExpandLabel(Pointer expand, FfiNameOrId.ByValue label);

    Pointer initEdgexpdOperator(Pointer expand, boolean isEdge);

    ResultCode appendEdgexpdOperator(Pointer plan, Pointer edgeXpd, int parent, IntByReference oprIdx);

    ResultCode setEdgexpdAlias(Pointer edgeXpd, FfiAlias.ByValue alias);

    Pointer initLimitOperator();

    ResultCode setLimitRange(Pointer limit, int lower, int upper);

    ResultCode appendLimitOperator(Pointer plan, Pointer limit, int parent, IntByReference oprIdx);

    Pointer initSelectOperator();

    ResultCode setSelectPredicate(Pointer select, String predicate);

    ResultCode appendSelectOperator(Pointer plan, Pointer select, int parent, IntByReference oprIdx);

    Pointer initOrderbyOperator();

    ResultCode addOrderbyPair(Pointer orderBy, FfiVariable.ByValue variable, FfiOrderOpt orderOpt);

    ResultCode setOrderbyLimit(Pointer orderBy, int lower, int upper);

    ResultCode appendOrderbyOperator(Pointer plan, Pointer orderBy, int parent, IntByReference oprIdx);

    Pointer initProjectOperator(boolean isAppend);

    ResultCode addProjectExprAlias(Pointer project, String expr, FfiAlias.ByValue alias);

    ResultCode appendProjectOperator(Pointer plan, Pointer project, int parent, IntByReference oprIdx);

    /// To initialize an As operator
    Pointer initAsOperator();

    /// Set the alias of the entity to As
    ResultCode setAsAlias(Pointer as, FfiAlias.ByValue alias);

    /// Append an As operator to the logical plan
    ResultCode appendAsOperator(Pointer plan, Pointer as, int parent, IntByReference oprIdx);

    Pointer initGroupbyOperator();

    ResultCode addGroupbyKeyAlias(Pointer groupBy, FfiVariable.ByValue key, FfiAlias.ByValue alias);

    ResultCode addGroupbyAggFn(Pointer groupBy, FfiAggFn.ByValue aggFn);

    ResultCode appendGroupbyOperator(Pointer plan, Pointer groupBy, int parent, IntByReference oprIdx);

    Pointer initDedupOperator();

    ResultCode addDedupKey(Pointer dedup, FfiVariable.ByValue var);

    ResultCode appendDedupOperator(Pointer plan, Pointer dedup, int parent, IntByReference oprIdx);

    Pointer initSinkOperator(boolean isCurrent);

    ResultCode addSinkColumn(Pointer sink, FfiNameOrId.ByValue column);

    ResultCode appendSinkOperator(Pointer plan, Pointer sink, int parent, IntByReference oprIdx);

    FfiNameOrId.ByValue noneNameOrId();

    FfiNameOrId.ByValue cstrAsNameOrId(String name);

    FfiConst.ByValue cstrAsConst(String value);

    FfiConst.ByValue int32AsConst(int value);

    FfiConst.ByValue int64AsConst(long value);

    FfiProperty.ByValue asNoneKey();

    FfiProperty.ByValue asLabelKey();

    FfiProperty.ByValue asIdKey();

    FfiProperty.ByValue asPropertyKey(FfiNameOrId.ByValue key);

    FfiVariable.ByValue asVarTagOnly(FfiNameOrId.ByValue tag);

    FfiVariable.ByValue asVarPropertyOnly(FfiProperty.ByValue property);

    FfiVariable.ByValue asVar(FfiNameOrId.ByValue tag, FfiProperty.ByValue property);

    FfiVariable.ByValue asNoneVar();

    FfiAggFn.ByValue initAggFn(FfiAggOpt aggregate, FfiAlias.ByValue alias);

    void destroyJobBuffer(FfiJobBuffer.ByValue value);

    ResultCode setSchema(String schemaJson);
}
