package com.netlink.api;

/**
 * @author lbl
 * @date: 2024/11/10
 */
public class Context {
    public static native void initializeAsyncRuntime();

    /**
     * https://crates.io/crates/log4rs
     * set log4rs.yaml
     *
     * @param logConfigFile file path
     */
    public static native void initializeLog(String logConfigFile);
}
