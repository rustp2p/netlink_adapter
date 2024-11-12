use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use combine::error::StringStreamError;
    use netlink_core::config::{Config as NetlinkConfig, ConfigBuilder, GroupCode};
    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::str::FromStr;

    use crate::java_bridge::utils::convert_jni_result;
    use robusta_jni::convert::{
        Field, IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue,
    };
    use robusta_jni::jni::errors::{Error, Result as JniResult};
    use robusta_jni::jni::objects::AutoLocal;
    use robusta_jni::jni::JNIEnv;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.netlink.api)]
    pub struct Config<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
        #[field]
        listenRoute: Field<'env, 'borrow, bool>,
        #[field]
        configName: Field<'env, 'borrow, String>,
        #[field]
        nodeIpv4: Field<'env, 'borrow, i32>,
        #[field]
        prefix: Field<'env, 'borrow, i8>,
        #[field]
        nodeIpv6: Field<'env, 'borrow, String>,
        #[field]
        prefixV6: Field<'env, 'borrow, i8>,
        #[field]
        tunName: Field<'env, 'borrow, String>,
        #[field]
        encrypt: Field<'env, 'borrow, String>,
        #[field]
        algorithm: Field<'env, 'borrow, String>,
        #[field]
        port: Field<'env, 'borrow, i16>,
        #[field]
        groupCode: Field<'env, 'borrow, String>,
        #[field]
        peer: Field<'env, 'borrow, Vec<String>>,
        #[field]
        bindDevName: Field<'env, 'borrow, String>,
        #[field]
        exitNode: Field<'env, 'borrow, i32>,
        #[field]
        udpStun: Field<'env, 'borrow, Vec<String>>,
        #[field]
        tcpStun: Field<'env, 'borrow, Vec<String>>,
    }
    impl<'env: 'borrow, 'borrow> Config<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {}
        pub extern "jni" fn check(self, _env: &JNIEnv) -> JniResult<()> {
            self.to_config()?;
            Ok(())
        }
        pub(crate) fn to_config(&self) -> JniResult<NetlinkConfig> {
            let listen_route = self.listenRoute.get()?;
            let config_name = convert_jni_result(self.configName.get())?;
            let node_ipv4 = Ipv4Addr::from(self.nodeIpv4.get()? as u32);
            let prefix = self.prefix.get()? as u8;
            let node_ipv6 = convert_jni_result(self.nodeIpv6.get())?;
            let node_ipv6 = if let Some(node_ipv6) = node_ipv6 {
                match Ipv6Addr::from_str(&node_ipv6) {
                    Ok(node_ipv6) => Some(node_ipv6),
                    Err(e) => Err(Error::ParseFailed(
                        StringStreamError::UnexpectedParse,
                        format!("node_ipv6: {e}"),
                    ))?,
                }
            } else {
                None
            };
            let prefix_v6 = self.prefixV6.get()? as u8;
            let tun_name = convert_jni_result(self.tunName.get())?;
            let encrypt = convert_jni_result(self.encrypt.get())?;
            let algorithm = convert_jni_result(self.algorithm.get())?;
            let port = self.port.get()? as u16;
            let group_code = self.groupCode.get()?;
            let group_code = GroupCode::try_from(group_code).map_err(|e| {
                Error::ParseFailed(
                    StringStreamError::UnexpectedParse,
                    format!("group_code: {e}"),
                )
            })?;
            let peer = convert_jni_result(self.peer.get())?;
            let bind_dev_name = convert_jni_result(self.bindDevName.get())?;
            let exit_node = self.exitNode.get()? as u32;
            let exit_node = if exit_node == 0 {
                None
            } else {
                Some(Ipv4Addr::from(exit_node))
            };
            let udp_stun = convert_jni_result(self.udpStun.get())?;
            let tcp_stun = convert_jni_result(self.tcpStun.get())?;
            let mut builder = ConfigBuilder::new()
                .config_name(config_name)
                .listen_route(listen_route)
                .node_ipv4(node_ipv4)
                .node_ipv6(node_ipv6)
                .prefix(prefix)
                .prefix_v6(prefix_v6)
                .tun_name(tun_name)
                .algorithm(algorithm)
                .encrypt(encrypt)
                .port(port)
                .group_code(group_code)
                .peer_str(peer)
                .map_err(|e| {
                    Error::ParseFailed(StringStreamError::UnexpectedParse, format!("peer: {e}"))
                })?
                .bind_dev_name(bind_dev_name)
                .exit_node(exit_node);
            if let Some(udp_stun) = udp_stun {
                if !udp_stun.is_empty() {
                    builder = builder.udp_stun(udp_stun)
                }
            }
            if let Some(tcp_stun) = tcp_stun {
                if !tcp_stun.is_empty() {
                    builder = builder.tcp_stun(tcp_stun)
                }
            }
            let config = builder.build().map_err(|e| {
                Error::ParseFailed(StringStreamError::UnexpectedParse, format!("peer: {e}"))
            })?;
            Ok(config)
        }
    }
}
