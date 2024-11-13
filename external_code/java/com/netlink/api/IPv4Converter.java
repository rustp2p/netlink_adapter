package com.netlink.api;

/**
 * @author lbl
 * @date: 2024/11/10
 */
public class IPv4Converter {
    public static int ipv4ToInt(String ipAddress) {
        String[] parts = ipAddress.split("\\.");
        int result = 0;
        for (int i = 0; i < 4; i++) {
            result |= (Integer.parseInt(parts[i]) << (24 - (i * 8)));
        }
        return result;
    }

    public static String intToIpv4(int ipInt) {
        return String.format("%d.%d.%d.%d",
                (ipInt >> 24) & 0xFF,
                (ipInt >> 16) & 0xFF,
                (ipInt >> 8) & 0xFF,
                ipInt & 0xFF);

    }
}
