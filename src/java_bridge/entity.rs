use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use netlink_core::api::entity::{
        GroupItem as NetlinkGroupItem, NetworkNatInfo as NetlinkNatInfo,
        RouteItem as NetlinkRouteItem,
    };

    use robusta_jni::convert::{
        Field, IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue,
    };
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::{AutoLocal, JObject};
    use robusta_jni::jni::JNIEnv;
    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.netlink.api)]
    pub struct RouteItem<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
        #[field]
        nodeId: Field<'env, 'borrow, i32>,
        #[field]
        nextHop: Field<'env, 'borrow, String>,
        #[field]
        protocol: Field<'env, 'borrow, String>,
        #[field]
        metric: Field<'env, 'borrow, i8>,
        #[field]
        rtt: Field<'env, 'borrow, i32>,
        #[field]
        nextInterface: Field<'env, 'borrow, String>,
    }
    impl<'env: 'borrow, 'borrow> RouteItem<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {}
        pub fn forget(self) -> JObject<'env> {
            self.raw.forget()
        }
        pub(crate) fn set(&mut self, route_item: NetlinkRouteItem) -> JniResult<()> {
            self.nodeId.set(u32::from(route_item.node_id) as _)?;
            self.nextHop.set(route_item.next_hop)?;
            self.protocol.set(route_item.protocol)?;
            self.metric.set(route_item.metric as _)?;
            self.rtt.set(route_item.rtt as _)?;
            self.nextInterface.set(route_item.interface)?;
            Ok(())
        }
    }
    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.netlink.api)]
    pub struct GroupItem<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
        #[field]
        groupCode: Field<'env, 'borrow, String>,
        #[field]
        nodeNum: Field<'env, 'borrow, i32>,
    }
    impl<'env: 'borrow, 'borrow> GroupItem<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {}
        pub fn forget(self) -> JObject<'env> {
            self.raw.forget()
        }
        pub(crate) fn set(&mut self, group_item: NetlinkGroupItem) -> JniResult<()> {
            self.groupCode.set(group_item.group_code)?;
            self.nodeNum.set(group_item.node_num as i32)?;
            Ok(())
        }
    }
    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.netlink.api)]
    pub struct NetworkNatInfo<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
        #[field]
        nodeIp: Field<'env, 'borrow, i32>,
        #[field]
        localIpv4: Field<'env, 'borrow, i32>,
        #[field]
        ipv6: Field<'env, 'borrow, String>,
        #[field]
        natType: Field<'env, 'borrow, String>,
        #[field]
        publicIps: Field<'env, 'borrow, Vec<i32>>,
        #[field]
        publicUdpPorts: Field<'env, 'borrow, Vec<i16>>,
        #[field]
        publicTcpPort: Field<'env, 'borrow, i16>,
        #[field]
        localUdpPorts: Field<'env, 'borrow, Vec<i16>>,
        #[field]
        localTcpPort: Field<'env, 'borrow, i16>,
    }
    impl<'env: 'borrow, 'borrow> NetworkNatInfo<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {}
        pub fn forget(self) -> JObject<'env> {
            self.raw.forget()
        }
        pub(crate) fn set(&mut self, info: NetlinkNatInfo) -> JniResult<()> {
            self.nodeIp.set(u32::from(info.node_ip) as _)?;
            self.localIpv4.set(u32::from(info.local_ipv4) as _)?;
            if let Some(ipv6) = info.ipv6.as_ref() {
                self.ipv6.set(ipv6.to_string())?;
            }
            self.natType.set(format!("{:?}", info.nat_type))?;
            self.publicIps.set(
                info.public_ips
                    .iter()
                    .map(|v| u32::from(*v) as i32)
                    .collect(),
            )?;
            self.publicUdpPorts
                .set(info.public_udp_ports.iter().map(|v| *v as i16).collect())?;
            self.publicTcpPort.set(info.public_tcp_port as _)?;
            self.localUdpPorts
                .set(info.local_udp_ports.iter().map(|v| *v as i16).collect())?;
            self.localTcpPort.set(info.local_tcp_port as _)?;
            Ok(())
        }
    }
}
