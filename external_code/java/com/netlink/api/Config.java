package com.netlink.api;

import java.util.ArrayList;

/**
 * @author lbl
 * @date: 2024/11/10
 */
public class Config {
    private boolean listenRoute = true;
    private String configName;
    private int nodeIpv4;
    private byte prefix;
    private String nodeIpv6;
    private byte prefixV6;
    private String tunName;
    private String encrypt;
    private String algorithm;
    private short port;
    private String groupCode;
    private ArrayList<String> peer = new ArrayList<>();
    private String bindDevName;
    private int exitNode;
    private ArrayList<String> udpStun = new ArrayList<>();
    private ArrayList<String> tcpStun = new ArrayList<>();
    public native void check();
    public boolean isListenRoute() {
        return listenRoute;
    }

    public void setListenRoute(boolean listenRoute) {
        this.listenRoute = listenRoute;
    }

    public String getConfigName() {
        return configName;
    }

    public void setConfigName(String configName) {
        this.configName = configName;
    }

    public int getNodeIpv4() {
        return nodeIpv4;
    }

    public void setNodeIpv4(int nodeIpv4) {
        this.nodeIpv4 = nodeIpv4;
    }

    public byte getPrefix() {
        return prefix;
    }

    public void setPrefix(byte prefix) {
        this.prefix = prefix;
    }

    public String getNodeIpv6() {
        return nodeIpv6;
    }

    public void setNodeIpv6(String nodeIpv6) {
        this.nodeIpv6 = nodeIpv6;
    }

    public byte getPrefixV6() {
        return prefixV6;
    }

    public void setPrefixV6(byte prefixV6) {
        this.prefixV6 = prefixV6;
    }

    public String getTunName() {
        return tunName;
    }

    public void setTunName(String tunName) {
        this.tunName = tunName;
    }

    public String getEncrypt() {
        return encrypt;
    }

    public void setEncrypt(String encrypt) {
        this.encrypt = encrypt;
    }

    public String getAlgorithm() {
        return algorithm;
    }

    public void setAlgorithm(String algorithm) {
        this.algorithm = algorithm;
    }

    public short getPort() {
        return port;
    }

    public void setPort(short port) {
        this.port = port;
    }

    public String getGroupCode() {
        return groupCode;
    }

    public void setGroupCode(String groupCode) {
        this.groupCode = groupCode;
    }

    public ArrayList<String> getPeer() {
        return peer;
    }

    public void setPeer(ArrayList<String> peer) {
        this.peer = peer;
    }

    public String getBindDevName() {
        return bindDevName;
    }

    public void setBindDevName(String bindDevName) {
        this.bindDevName = bindDevName;
    }

    public int getExitNode() {
        return exitNode;
    }

    public void setExitNode(int exitNode) {
        this.exitNode = exitNode;
    }

    public ArrayList<String> getUdpStun() {
        return udpStun;
    }

    public void setUdpStun(ArrayList<String> udpStun) {
        this.udpStun = udpStun;
    }

    public ArrayList<String> getTcpStun() {
        return tcpStun;
    }

    public void setTcpStun(ArrayList<String> tcpStun) {
        this.tcpStun = tcpStun;
    }
}
