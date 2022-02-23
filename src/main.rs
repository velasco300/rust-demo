pub(crate) mod controller;
pub mod middleware;

use crate::controller::user_controller;
use axum::{
    handler::Handler, http::Request, middleware::from_fn, response::IntoResponse, routing::get,
    Json, Router, Server,
};
use rust_demo_core::common::RspResult;
use std::net::SocketAddr;
use tracing::info;
use tracing_appender::rolling;

#[tokio::main]
async fn main() {
    let (non_blocking, _guard) =
        tracing_appender::non_blocking(rolling::daily("./logs", "app.log"));
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_line_number(true)
        .with_writer(non_blocking)
        .init();
    info!("main start");
    let app = Router::new()
        .route(
            "/users",
            get(user_controller::index).post(user_controller::add),
        )
        .route(
            "/users/:id",
            get(user_controller::show)
                .delete(user_controller::delete)
                .put(user_controller::update),
        )
        .route("/users/search", get(user_controller::pg_by_age_and_egin))
        .layer(from_fn(middleware::handle_global_exception))
        .fallback(defalut_route.into_service());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn defalut_route<B>(req: Request<B>) -> impl IntoResponse {
    let method = req.method().as_str();
    let url = req.uri().path();
    Json(RspResult::fail(
        "没有找到匹配的URL".to_owned(),
        format!("{} : {}", method, url),
    ))
}
