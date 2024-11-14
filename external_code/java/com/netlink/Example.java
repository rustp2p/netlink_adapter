package com.netlink;

import com.netlink.api.*;

import java.io.IOException;
import java.util.List;

/**
 * @author lbl
 * @date: 2024/11/10
 */
public class Example {
    static {
        System.loadLibrary("netlink_adapter");
    }

    public static void main(String[] args) throws IOException, InterruptedException {
        Context.initializeLog("log4rs.yaml");
        Context.initializeAsyncRuntime();
        Config config = new Config();
        config.setListenRoute(true);
        config.setNodeIpv4(IPv4Converter.ipv4ToInt("10.26.1.13"));
        config.setPort((short) 23333);
        config.setGroupCode("test_group_code");

        config.check();
        try (NetlinkCoreApi netlinkCoreApi = new NetlinkCoreApi()) {
            netlinkCoreApi.open(config);
            Thread.sleep(2000);
            List<GroupItem> routeItems = netlinkCoreApi.groups();
            System.out.println(routeItems);
            NetworkNatInfo networkNatInfo = netlinkCoreApi.currentInfo();
            System.out.println(networkNatInfo);
        }
        System.out.println("stopped");
    }
}
