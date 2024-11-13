package com.netlink.api;

import java.util.ArrayList;

/**
 * @author lbl
 * @date: 2024/11/10
 */
public class NetworkNatInfo {
    private int nodeIp;
    private int localIpv4;
    private String ipv6;
    private String natType;
    private ArrayList<Integer> publicIps;
    private ArrayList<Short> publicUdpPorts;
    private short publicTcpPort;
    private ArrayList<Short> localUdpPorts;
    private short localTcpPort;

    public int getNodeIp() {
        return nodeIp;
    }

    public void setNodeIp(int nodeIp) {
        this.nodeIp = nodeIp;
    }

    public int getLocalIpv4() {
        return localIpv4;
    }

    public void setLocalIpv4(int localIpv4) {
        this.localIpv4 = localIpv4;
    }

    public String getIpv6() {
        return ipv6;
    }

    public void setIpv6(String ipv6) {
        this.ipv6 = ipv6;
    }

    public String getNatType() {
        return natType;
    }

    public void setNatType(String natType) {
        this.natType = natType;
    }

    public ArrayList<Integer> getPublicIps() {
        return publicIps;
    }

    public void setPublicIps(ArrayList<Integer> publicIps) {
        this.publicIps = publicIps;
    }

    public ArrayList<Short> getPublicUdpPorts() {
        return publicUdpPorts;
    }

    public void setPublicUdpPorts(ArrayList<Short> publicUdpPorts) {
        this.publicUdpPorts = publicUdpPorts;
    }

    public short getPublicTcpPort() {
        return publicTcpPort;
    }

    public void setPublicTcpPort(short publicTcpPort) {
        this.publicTcpPort = publicTcpPort;
    }

    public ArrayList<Short> getLocalUdpPorts() {
        return localUdpPorts;
    }

    public void setLocalUdpPorts(ArrayList<Short> localUdpPorts) {
        this.localUdpPorts = localUdpPorts;
    }

    public short getLocalTcpPort() {
        return localTcpPort;
    }

    public void setLocalTcpPort(short localTcpPort) {
        this.localTcpPort = localTcpPort;
    }

    @Override
    public String toString() {
        return "NetworkNatInfo{" +
                "nodeIp=" + IPv4Converter.intToIpv4(nodeIp) +
                ", localIpv4=" + IPv4Converter.intToIpv4(localIpv4) +
                ", ipv6='" + ipv6 + '\'' +
                ", natType='" + natType + '\'' +
                ", publicIps=" + publicIps +
                ", publicUdpPorts=" + publicUdpPorts +
                ", publicTcpPort=" + publicTcpPort +
                ", localUdpPorts=" + localUdpPorts +
                ", localTcpPort=" + localTcpPort +
                '}';
    }
}
