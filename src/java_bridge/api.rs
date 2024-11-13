use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use combine::error::StringStreamError;
    use robusta_jni::convert::{
        Field, IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue,
    };
    use robusta_jni::jni::errors::{Error, Result as JniResult};
    use robusta_jni::jni::objects::{AutoLocal, JObject};
    use robusta_jni::jni::JNIEnv;

    use crate::initialize_async_runtime;
    use crate::java_bridge::config::jni::Config;
    use crate::java_bridge::entity::jni::{GroupItem, NetworkNatInfo, RouteItem};
    use crate::java_bridge::utils::async_runtime;
    use netlink_core::api::NetLinkCoreApi as Api;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.netlink.api)]
    pub struct Context<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }
    impl<'env: 'borrow, 'borrow> Context<'env, 'borrow> {
        pub extern "jni" fn initializeAsyncRuntime() {
            initialize_async_runtime();
        }
        pub extern "jni" fn initializeLog(log_file: String) -> JniResult<()> {
            if let Err(e) = log4rs::init_file(log_file, Default::default()) {
                Err(Error::ParseFailed(
                    StringStreamError::UnexpectedParse,
                    format!("open: {e}"),
                ))?
            }
            Ok(())
        }
    }
    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.netlink.api)]
    pub struct NetlinkCoreApi<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
        #[field]
        api: Field<'env, 'borrow, i64>,
    }
    impl<'env: 'borrow, 'borrow> NetlinkCoreApi<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {}
        pub extern "jni" fn open(
            mut self,
            _env: &JNIEnv,
            config: Config<'env, 'borrow>,
        ) -> JniResult<()> {
            let config = config.to_config()?;
            let runtime = async_runtime()?;
            let api = match runtime.block_on(Api::open(config)) {
                Ok(api) => api,
                Err(e) => Err(Error::ParseFailed(
                    StringStreamError::UnexpectedParse,
                    format!("open: {e:?}"),
                ))?,
            };
            self.set_api(api)
        }
        fn set_api(&mut self, api: Api) -> JniResult<()> {
            let raw = Box::into_raw(Box::new(api));
            match self.api.set(raw as _) {
                Ok(_) => {}
                Err(e) => {
                    _ = unsafe { Box::from_raw(raw) };
                    Err(e)?
                }
            };
            Ok(())
        }

        #[cfg(unix)]
        pub extern "jni" fn openWithTun(
            mut self,
            _env: &JNIEnv,
            config: Config<'env, 'borrow>,
            tunFd: u32,
        ) -> JniResult<()> {
            let config = config.to_config()?;
            let runtime = async_runtime()?;
            let api = match runtime.block_on(unsafe { Api::open_with_tun_fd(config, tunFd) }) {
                Ok(api) => api,
                Err(e) => Err(Error::ParseFailed(
                    StringStreamError::UnexpectedParse,
                    format!("openWithTun: {e:?}"),
                ))?,
            };
            self.set_api(api)
        }
        pub extern "jni" fn close(mut self, _env: &JNIEnv) -> JniResult<()> {
            if let Ok(api) = self.api.get() {
                if api != 0 {
                    self.api.set(0)?;
                    let raw = api as *mut Api;
                    _ = unsafe { Box::from_raw(raw) };
                }
            }
            Ok(())
        }
        fn get(&self) -> JniResult<&Api> {
            if let Ok(api) = self.api.get() {
                let raw = unsafe { &*(api as *mut Api) };
                Ok(raw)
            } else {
                Err(Error::NullPtr("not open"))
            }
        }
        pub extern "jni" fn currentNodes(
            self,
            env: &'borrow JNIEnv<'env>,
        ) -> JniResult<Vec<JObject<'env>>> {
            let api = self.get()?;
            let vec = api.current_nodes().map_err(|e| {
                Error::ParseFailed(
                    StringStreamError::UnexpectedParse,
                    format!("currentNodes: {e:?}"),
                )
            })?;
            let mut rs = Vec::with_capacity(vec.len());
            for x in vec {
                let mut item = RouteItem::new(env)?;
                item.set(x)?;
                rs.push(item.forget());
            }
            Ok(rs)
        }
        pub extern "jni" fn nodesByGroup(
            self,
            env: &'borrow JNIEnv<'env>,
            group_code: String,
        ) -> JniResult<Vec<JObject<'env>>> {
            let api = self.get()?;
            let vec = api.nodes_by_group(&group_code).map_err(|e| {
                Error::ParseFailed(
                    StringStreamError::UnexpectedParse,
                    format!("nodesByGroup: {e:?}"),
                )
            })?;
            let mut rs = Vec::with_capacity(vec.len());
            for x in vec {
                let mut item = RouteItem::new(env)?;
                item.set(x)?;
                rs.push(item.forget());
            }
            Ok(rs)
        }
        pub extern "jni" fn groups(
            self,
            env: &'borrow JNIEnv<'env>,
        ) -> JniResult<Vec<JObject<'env>>> {
            let api = self.get()?;
            let vec = api.groups().map_err(|e| {
                Error::ParseFailed(StringStreamError::UnexpectedParse, format!("groups: {e:?}"))
            })?;
            let mut rs = Vec::with_capacity(vec.len());
            for x in vec {
                let mut item = GroupItem::new(env)?;
                item.set(x)?;
                rs.push(item.forget());
            }
            Ok(rs)
        }
        pub extern "jni" fn currentInfo(
            self,
            env: &'borrow JNIEnv<'env>,
        ) -> JniResult<JObject<'env>> {
            let api = self.get()?;
            let info = api.current_info().map_err(|e| {
                Error::ParseFailed(StringStreamError::UnexpectedParse, format!("groups: {e:?}"))
            })?;
            let mut item = NetworkNatInfo::new(env)?;
            item.set(info)?;
            Ok(item.forget())
        }
    }
}
