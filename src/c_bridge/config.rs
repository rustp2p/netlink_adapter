use std::ffi::CStr;

use libc::{c_char, c_uchar, c_uint, c_ushort};
use netlink_core::config::{Config, ConfigBuilder};

#[repr(C)]
pub struct CConfig {
    listen_route: bool,
    config_name: *mut c_char,
    node_ipv4: c_uint,
    node_ipv6: *mut u8, // len = 16
    prefix: c_uchar,
    prefix_v6: c_uchar,
    tun_name: *mut c_char,
    encrypt: *mut c_char,
    algorithm: *mut c_char,
    port: c_ushort,
    group_code: *mut c_char,
    peer: *mut *mut c_char,
    peer_count: c_uint,
    bind_dev_name: *mut c_char,
    exit_node: c_uint,
    udp_stun: *mut *mut c_char,
    udp_stun_count: c_uint,
    tcp_stun: *mut *mut c_char,
    tcp_stun_count: c_uint,
}

pub(crate) unsafe fn to_config(c_config: &CConfig) -> anyhow::Result<Config> {
    let config_name = if !c_config.config_name.is_null() {
        Some(
            CStr::from_ptr(c_config.config_name)
                .to_string_lossy()
                .into_owned(),
        )
    } else {
        None
    };
    if c_config.group_code.is_null() {
        Err(anyhow::anyhow!(
            "Received null pointer for CConfig group_code"
        ))?;
    }
    let group_code = CStr::from_ptr(c_config.group_code)
        .to_string_lossy()
        .into_owned();
    let tun_name = if !c_config.tun_name.is_null() {
        Some(
            CStr::from_ptr(c_config.tun_name)
                .to_string_lossy()
                .into_owned(),
        )
    } else {
        None
    };

    let encrypt = if !c_config.encrypt.is_null() {
        Some(
            CStr::from_ptr(c_config.encrypt)
                .to_string_lossy()
                .into_owned(),
        )
    } else {
        None
    };

    let algorithm = if !c_config.algorithm.is_null() {
        Some(
            CStr::from_ptr(c_config.algorithm)
                .to_string_lossy()
                .into_owned(),
        )
    } else {
        None
    };
    let bind_dev_name = if !c_config.bind_dev_name.is_null() {
        Some(
            CStr::from_ptr(c_config.bind_dev_name)
                .to_string_lossy()
                .into_owned(),
        )
    } else {
        None
    };

    let peer = if !c_config.peer.is_null() {
        let mut peers = Vec::new();
        for i in 0..c_config.peer_count {
            let peer_ptr = *c_config.peer.offset(i as isize);
            if !peer_ptr.is_null() {
                peers.push(CStr::from_ptr(peer_ptr).to_string_lossy().into_owned());
            }
        }
        Some(peers)
    } else {
        None
    };

    let udp_stun = if !c_config.udp_stun.is_null() {
        let mut stuns = Vec::new();
        for i in 0..c_config.udp_stun_count {
            let stun_ptr = *c_config.udp_stun.offset(i as isize);
            if !stun_ptr.is_null() {
                stuns.push(CStr::from_ptr(stun_ptr).to_string_lossy().into_owned());
            }
        }
        Some(stuns)
    } else {
        None
    };

    let tcp_stun = if !c_config.tcp_stun.is_null() {
        let mut stuns = Vec::new();
        for i in 0..c_config.tcp_stun_count {
            let stun_ptr = *c_config.tcp_stun.offset(i as isize);
            if !stun_ptr.is_null() {
                stuns.push(CStr::from_ptr(stun_ptr).to_string_lossy().into_owned());
            }
        }
        Some(stuns)
    } else {
        None
    };

    let exit_node = if c_config.exit_node == 0 {
        None
    } else {
        Some(c_config.exit_node.into())
    };
    let node_ipv6 = if c_config.node_ipv6.is_null() {
        None
    } else {
        unsafe {
            let node_ipv6 = std::slice::from_raw_parts_mut(c_config.node_ipv6, 16);
            let node_ipv6: [u8; 16] = node_ipv6.try_into().unwrap();
            Some(node_ipv6.into())
        }
    };
    let mut builder = ConfigBuilder::new()
        .listen_route(c_config.listen_route)
        .node_ipv4(c_config.node_ipv4.into())
        .exit_node(exit_node)
        .prefix(c_config.prefix)
        .node_ipv6(node_ipv6)
        .prefix_v6(c_config.prefix_v6)
        .group_code(group_code.try_into()?)
        .port(c_config.port)
        .algorithm(algorithm)
        .encrypt(encrypt)
        .config_name(config_name)
        .tun_name(tun_name)
        .bind_dev_name(bind_dev_name)
        .peer_str(peer)?;
    if let Some(udp_stun) = udp_stun {
        builder = builder.udp_stun(udp_stun)
    }
    if let Some(tcp_stun) = tcp_stun {
        builder = builder.tcp_stun(tcp_stun)
    }
    builder.build()
}
