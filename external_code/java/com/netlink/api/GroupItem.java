package com.netlink.api;

/**
 * @author lbl
 * @date: 2024/11/10
 */
public class GroupItem {
    private String groupCode;
    private int nodeNum;

    public GroupItem() {
    }

    public String getGroupCode() {
        return groupCode;
    }

    public void setGroupCode(String groupCode) {
        this.groupCode = groupCode;
    }

    public int getNodeNum() {
        return nodeNum;
    }

    public void setNodeNum(int nodeNum) {
        this.nodeNum = nodeNum;
    }

    @Override
    public String toString() {
        return "GroupItem{" +
                "groupCode='" + groupCode + '\'' +
                ", nodeNum=" + nodeNum +
                '}';
    }
}
