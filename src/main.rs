pub(crate) mod controller;
pub mod middleware;

use crate::controller::user_controller;
use axum::{
    handler::Handler, http::Request, response::IntoResponse, routing::get, Json, Router, Server,
};
use rust_demo_core::common::RspResult;
use std::net::SocketAddr;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rust_demo=debug,rust_demo_core=debug")
    }
    let (d_non_blocking, _guard) =
        tracing_appender::non_blocking(rolling::minutely("./logs", "debug"));
    let (e_non_blocking, _guard) =
        tracing_appender::non_blocking(rolling::daily("./logs", "error"));
    tracing_subscriber::fmt()
        .with_writer(d_non_blocking.and(e_non_blocking))
        .with_ansi(false)
        .with_max_level(tracing::Level::TRACE)
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
