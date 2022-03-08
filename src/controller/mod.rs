pub mod car_controller;
pub mod user_controller;

use rust_demo_core::common::{AppErr, RspResult};
use tracing::info;

pub fn make_result<T>(rs: Result<T, anyhow::Error>) -> RspResult<Option<T>> {
    match rs {
        Ok(data) => RspResult::success(Some(data)),
        Err(e) => {
            if let Some(app_err) = e.downcast_ref::<AppErr>() {
                RspResult::fail(app_err.msg.clone(), None)
            } else {
                info!("{:?}", e);
                RspResult::fail("系统错误,请联系管理员!!!".to_owned(), None)
            }
        }
    }
}
