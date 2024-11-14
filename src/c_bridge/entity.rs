use std::ffi::CString;

use libc::{c_char, c_uchar, c_uint, c_ushort};

use netlink_core::api::entity::{GroupItem, NetworkNatInfo, RouteItem};

#[repr(C)]
pub struct CRouteItem {
    pub node_id: c_uint,
    pub next_hop: *mut c_char,
    pub protocol: *mut c_char,
    pub metric: c_uchar,
    pub rtt: c_uint,
    pub interface: *mut c_char,
}

#[repr(C)]
pub struct CRouteItemVec {
    ptr: *mut CRouteItem,
    length: c_uint,
}
impl Drop for CRouteItem {
    fn drop(&mut self) {
        unsafe {
            if !self.next_hop.is_null() {
                _ = CString::from_raw(self.next_hop);
            }
            if !self.protocol.is_null() {
                _ = CString::from_raw(self.protocol);
            }
            if !self.interface.is_null() {
                _ = CString::from_raw(self.interface);
            }
        }
    }
}
impl Drop for CRouteItemVec {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                _ = Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    self.ptr,
                    self.length as usize,
                ));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_CRouteItemVec(route_items: *mut CRouteItemVec) {
    if !route_items.is_null() {
        unsafe { drop(Box::from_raw(route_items)) }
    }
}
pub(crate) fn to_c_route_list(list: Vec<RouteItem>) -> CRouteItemVec {
    let mut route_items = Vec::with_capacity(list.len());
    for route in list {
        let c = CRouteItem {
            node_id: route.node_id.into(),
            next_hop: CString::new(route.next_hop)
                .expect("CString::new failed")
                .into_raw(),
            protocol: CString::new(route.protocol)
                .expect("CString::new failed")
                .into_raw(),
            metric: 0,
            rtt: 0,
            interface: CString::new(route.interface)
                .expect("CString::new failed")
                .into_raw(),
        };
        route_items.push(c);
    }
    let route_items = route_items.into_boxed_slice();
    let length = route_items.len() as _;
    CRouteItemVec {
        ptr: Box::into_raw(route_items) as _,
        length,
    }
}

#[repr(C)]
pub struct CGroupItem {
    pub group_code: *mut c_char,
    pub node_num: c_uint,
}
#[repr(C)]
pub struct CGroupItemVec {
    ptr: *mut CGroupItem,
    length: c_uint,
}
impl Drop for CGroupItem {
    fn drop(&mut self) {
        unsafe {
            if !self.group_code.is_null() {
                _ = CString::from_raw(self.group_code)
            }
        }
    }
}
impl Drop for CGroupItemVec {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                _ = Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    self.ptr,
                    self.length as usize,
                ));
            }
        }
    }
}
pub(crate) fn to_c_group_list(list: Vec<GroupItem>) -> CGroupItemVec {
    let mut group_list = Vec::with_capacity(list.len());
    for x in list {
        let c = CGroupItem {
            group_code: CString::new(x.group_code)
                .expect("CString::new failed")
                .into_raw(),
            node_num: x.node_num as _,
        };
        group_list.push(c);
    }
    let group_list = group_list.into_boxed_slice();
    let length = group_list.len() as _;
    CGroupItemVec {
        ptr: Box::into_raw(group_list) as _,
        length,
    }
}
#[no_mangle]
pub extern "C" fn drop_CGroupItemVec(list: *mut CGroupItemVec) {
    if !list.is_null() {
        unsafe { drop(Box::from_raw(list)) }
    }
}

#[repr(C)]
pub struct CNetworkNatInfo {
    node_ip: c_uint,
    local_ipv4: c_uint,
    ipv6: *mut c_char,
    nat_type: *mut c_char,
    public_ips: *mut c_uint,
    public_ips_len: c_uint,
    public_udp_ports: *mut c_ushort,
    public_udp_ports_len: c_uint,
    public_tcp_port: c_ushort,
    local_udp_ports: *mut c_ushort,
    local_udp_ports_len: c_uint,
    local_tcp_port: c_ushort,
}
impl Drop for CNetworkNatInfo {
    fn drop(&mut self) {
        unsafe {
            // Free the individual strings

            if !self.ipv6.is_null() {
                _ = CString::from_raw(self.ipv6)
            }
            if !self.nat_type.is_null() {
                _ = CString::from_raw(self.nat_type)
            }

            // Free the array of public_ips
            if !self.public_ips.is_null() {
                _ = Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    self.public_ips,
                    self.public_ips_len as usize,
                ));
            }

            // Free the array of public_udp_ports
            if !self.public_udp_ports.is_null() {
                _ = Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    self.public_udp_ports,
                    self.public_udp_ports_len as usize,
                ));
            }

            // Free the array of local_udp_ports
            if !self.local_udp_ports.is_null() {
                _ = Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    self.local_udp_ports,
                    self.local_udp_ports_len as usize,
                ));
            }
        }
    }
}
#[no_mangle]
pub extern "C" fn drop_CNetworkNatInfo(p: *mut CNetworkNatInfo) {
    unsafe {
        if !p.is_null() {
            drop(Box::from_raw(p));
        }
    }
}
pub(crate) fn to_c_network_info(info: NetworkNatInfo) -> CNetworkNatInfo {
    let ipv6 = if let Some(ipv6) = info.ipv6 {
        CString::new(ipv6.to_string())
            .expect("CString::new failed")
            .into_raw()
    } else {
        std::ptr::null_mut()
    };
    let public_ips_c: Vec<c_uint> = info.public_ips.into_iter().map(|s| u32::from(s)).collect();
    let public_ips = public_ips_c.into_boxed_slice();
    let public_ips_len = public_ips.len() as _;
    let public_udp_ports = info.public_udp_ports.into_boxed_slice();
    let public_udp_ports_len = public_udp_ports.len() as _;
    let local_udp_ports = info.local_udp_ports.into_boxed_slice();
    let local_udp_ports_len = local_udp_ports.len() as _;
    CNetworkNatInfo {
        node_ip: u32::from(info.node_ip),
        local_ipv4: u32::from(info.local_ipv4),
        ipv6,
        nat_type: CString::new(format!("{:?}", info.nat_type))
            .expect("CString::new failed")
            .into_raw(),
        public_ips: Box::into_raw(public_ips) as _,
        public_ips_len,
        public_udp_ports: Box::into_raw(public_udp_ports) as _,
        public_udp_ports_len,
        public_tcp_port: 0,
        local_udp_ports: Box::into_raw(local_udp_ports) as _,
        local_udp_ports_len,
        local_tcp_port: info.local_tcp_port,
    }
}
