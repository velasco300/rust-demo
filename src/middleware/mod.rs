use axum::{http::Request, middleware::Next, response::IntoResponse, Json};
use rust_demo_core::common::RspResult;
use tracing::{error, info};

pub async fn handle_global_exception<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = time::Instant::now();
    let mut rs = next.run(req).await;
    if !rs.status().is_success() {
        error!("{:?}", rs);
        rs = Json(RspResult::fail("未知错误,请联系管理员!".to_owned(), "")).into_response();
    };
    info!(
        "用时 {:?} 秒",
        (time::Instant::now() - start).as_seconds_f32()
    );
    rs
}
