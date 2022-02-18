pub(crate) mod controller;

use crate::controller::user_controller;
use axum::{handler::Handler, response::IntoResponse, routing::get, Json, Router, Server};
use rust_demo_core::common::RspResult;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rust_demo=debug,rust_demo_core=debug")
    }
    tracing_subscriber::fmt::init();
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
        .fallback(defalut_route.into_service());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn defalut_route() -> impl IntoResponse {
    Json(RspResult::fail("没有找到匹配的URL".to_owned(), ""))
}
