package com.netlink.api;

import java.io.Closeable;
import java.util.ArrayList;

/**
 * @author lbl
 * @date: 2024/11/10
 */
public class NetlinkCoreApi implements Closeable {
    private long api;

    /**
     * Prerequisite: call Context.initializeAsyncRuntime
     *
     * @param config config
     */
    public NetlinkCoreApi(Config config) {
        open(config);
    }

    /**
     * Only supports Unix systems
     *
     * @param config config
     * @param tunFd  tunFd
     */
    public NetlinkCoreApi(Config config, int tunFd) {
        openWithTun(config, tunFd);
    }

    private native void open(Config config);

    private native void openWithTun(Config config, int tunFd);

    public native void close();

    public native boolean isShutdownComplete();

    public native void waitShutdownComplete();

    /**
     * Waiting to stop
     *
     * @param time waiting time (ms)
     * @return is shutdown complete
     */
    public native boolean waitShutdownCompleteTimeout(long time);

    public native ArrayList<RouteItem> currentNodes();

    public native ArrayList<RouteItem> nodesByGroup(String groupCode);

    public native ArrayList<GroupItem> groups();

    public native NetworkNatInfo currentInfo();
}
