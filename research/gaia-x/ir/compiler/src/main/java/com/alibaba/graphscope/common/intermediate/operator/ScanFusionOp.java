package com.alibaba.graphscope.common.intermediate.operator;

import java.util.Optional;
import java.util.function.Function;

public class ScanFusionOp extends BaseOp {
    public ScanFusionOp(Function transformer) {
        super(transformer);
        scanOpt = Optional.empty();
        labels = Optional.empty();
        properties = Optional.empty();
        ids = Optional.empty();
        predicate = Optional.empty();
        limit = Optional.empty();
    }

    // vertex or edge
    private Optional<OpArg> scanOpt;

    // scan by labels
    private Optional<OpArg> labels;

    // scan by properties
    private Optional<OpArg> properties;

    // indexing based on id
    private Optional<OpArg> ids;

    // filter by predicate
    private Optional<OpArg> predicate;

    // filter by limit
    private Optional<OpArg> limit;

    public Optional<OpArg> getScanOpt() {
        return scanOpt;
    }

    public Optional<OpArg> getLabels() {
        return labels;
    }

    public Optional<OpArg> getProperties() {
        return properties;
    }

    public Optional<OpArg> getIds() {
        return ids;
    }

    public Optional<OpArg> getPredicate() {
        return predicate;
    }

    public Optional<OpArg> getLimit() {
        return limit;
    }

    public void setScanOpt(OpArg scanOpt) {
        this.scanOpt = Optional.of(scanOpt);
    }

    public void setLabels(OpArg labels) {
        this.labels = Optional.of(labels);
    }

    public void setProperties(OpArg properties) {
        this.properties = Optional.of(properties);
    }

    public void setIds(OpArg ids) {
        this.ids = Optional.of(ids);
    }

    public void setPredicate(OpArg predicate) {
        this.predicate = Optional.of(predicate);
    }

    public void setLimit(OpArg limit) {
        this.limit = Optional.of(limit);
    }
}
