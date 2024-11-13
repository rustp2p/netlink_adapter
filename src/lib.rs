use std::sync::{Arc, OnceLock};
use std::thread;
use tokio::runtime::Runtime;

#[cfg(feature = "c_bindings")]
pub mod c_bridge;
#[cfg(feature = "java_bindings")]
pub mod java_bridge;

static RUNTIME: OnceLock<Arc<Runtime>> = OnceLock::new();

pub fn initialize_async_runtime() {
    RUNTIME.get_or_init(|| {
        let rt = Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
        let rt1 = rt.clone();
        thread::spawn(move || {
            rt1.block_on(async {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(u64::MAX)).await;
                }
            });
        });
        rt
    });
}
