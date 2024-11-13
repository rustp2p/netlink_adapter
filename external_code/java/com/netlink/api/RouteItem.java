package com.netlink.api;

/**
 * @author lbl
 * @date: 2024/11/10
 */
public class RouteItem {
    private int nodeId;
    private String nextHop;
    private String protocol;
    private byte metric;
    private int rtt;
    private String nextInterface;
    public RouteItem() {
    }
    public int getNodeId() {
        return nodeId;
    }

    public void setNodeId(int nodeId) {
        this.nodeId = nodeId;
    }

    public String getNextHop() {
        return nextHop;
    }

    public void setNextHop(String nextHop) {
        this.nextHop = nextHop;
    }

    public String getProtocol() {
        return protocol;
    }

    public void setProtocol(String protocol) {
        this.protocol = protocol;
    }

    public byte getMetric() {
        return metric;
    }

    public void setMetric(byte metric) {
        this.metric = metric;
    }

    public int getRtt() {
        return rtt;
    }

    public void setRtt(int rtt) {
        this.rtt = rtt;
    }

    public String getNextInterface() {
        return nextInterface;
    }

    public void setNextInterface(String nextInterface) {
        this.nextInterface = nextInterface;
    }

    @Override
    public String toString() {
        return "RouteItem{" +
                "nodeId=" + IPv4Converter.intToIpv4(nodeId) +
                ", nextHop='" + nextHop + '\'' +
                ", protocol='" + protocol + '\'' +
                ", metric=" + metric +
                ", rtt=" + rtt +
                ", nextInterface='" + nextInterface + '\'' +
                '}';
    }
}
