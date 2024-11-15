use std::ffi::CString;

use crate::c_bridge::entity::{
    to_c_group_list, to_c_network_info, to_c_route_list, CGroupItemVec, CNetworkNatInfo,
    CRouteItemVec,
};
use crate::c_bridge::{to_config, CConfig};
use crate::RUNTIME;
use netlink_core::api::NetLinkCoreApi;
#[no_mangle]
pub extern "C" fn initialize_async_runtime() {
    crate::initialize_async_runtime();
}

#[no_mangle]
pub extern "C" fn create_netlink_api(config: *const CConfig) -> *mut NetLinkCoreApi {
    let config = match unsafe { to_config(&*config) } {
        Ok(config) => config,
        Err(e) => {
            log::warn!("create_api to_config {e:?}");
            return std::ptr::null_mut();
        }
    };
    let runtime = RUNTIME.get().expect("not initialize runtime");
    let api = match runtime.block_on(NetLinkCoreApi::open(config)) {
        Ok(rs) => rs,
        Err(e) => {
            log::warn!("{e:?}");
            return std::ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(api))
}
#[cfg(unix)]
#[no_mangle]
pub extern "C" fn create_netlink_api_with_fd(
    config: *mut CConfig,
    tun_fd: libc::c_uint,
) -> *mut NetLinkCoreApi {
    let config = match unsafe { to_config(&*config) } {
        Ok(config) => config,
        Err(e) => {
            log::warn!("create_api_with_fd to_config {e:?}");
            return std::ptr::null_mut();
        }
    };
    let runtime = RUNTIME.get().expect("not initialize runtime");
    let api = match runtime.block_on(unsafe { NetLinkCoreApi::open_with_tun_fd(config, tun_fd) }) {
        Ok(rs) => rs,
        Err(e) => {
            log::warn!("{e:?}");
            return std::ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(api))
}
#[no_mangle]
pub extern "C" fn drop_netlink_api(api: *mut NetLinkCoreApi) {
    if !api.is_null() {
        unsafe {
            let raw = Box::from_raw(api);
            drop(raw);
        }
    }
}
#[no_mangle]
pub extern "C" fn get_current_nodes(api: *mut NetLinkCoreApi) -> *mut CRouteItemVec {
    if api.is_null() {
        return std::ptr::null_mut();
    }
    let rs = unsafe { (&*api).current_nodes() };
    match rs {
        Ok(rs) => Box::into_raw(Box::new(to_c_route_list(rs))),
        Err(e) => {
            log::warn!("current_nodes {e:?}");
            return std::ptr::null_mut();
        }
    }
}
#[no_mangle]
pub extern "C" fn get_nodes_by_group(
    api: *mut NetLinkCoreApi,
    group_code: *mut libc::c_char,
) -> *mut CRouteItemVec {
    let group_code = unsafe { CString::from_raw(group_code) };
    let group_code = group_code.to_string_lossy().into_owned();
    let rs = unsafe { (&*api).nodes_by_group(&group_code) };
    match rs {
        Ok(rs) => Box::into_raw(Box::new(to_c_route_list(rs))),
        Err(e) => {
            log::warn!("nodes_by_group {e:?}");
            return std::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub extern "C" fn get_groups(api: *mut NetLinkCoreApi) -> *mut CGroupItemVec {
    let rs = unsafe { (&*api).groups() };
    match rs {
        Ok(rs) => Box::into_raw(Box::new(to_c_group_list(rs))),
        Err(e) => {
            log::warn!("groups {e:?}");
            return std::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub extern "C" fn get_current_info(api: *mut NetLinkCoreApi) -> *mut CNetworkNatInfo {
    let rs = unsafe { (&*api).current_info() };
    match rs {
        Ok(rs) => Box::into_raw(Box::new(to_c_network_info(rs))),
        Err(e) => {
            log::warn!("current_info {e:?}");
            return std::ptr::null_mut();
        }
    }
}
