package com.alibaba.graphscope.common.jna.type;

import com.alibaba.graphscope.common.jna.IntEnum;

public enum FfiVOpt implements IntEnum<FfiVOpt> {
    Start,
    End,
    Other;

    @Override
    public int getInt() {
        return this.ordinal();
    }

    @Override
    public FfiVOpt getEnum(int i) {
        FfiVOpt opts[] = values();
        if (i < opts.length && i >= 0) {
            return opts[i];
        }
        return null;
    }
}
