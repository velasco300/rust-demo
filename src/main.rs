pub(crate) mod controller;
pub mod middleware;

use crate::controller::user_controller;
use axum::{
    handler::Handler, http::Request, middleware::from_fn, response::IntoResponse, routing::get,
    Json, Router, Server,
};
use rust_demo_core::{
    common::RspResult,
    orm,
    util::{LogTimer, APP_CFG},
};
use std::net::SocketAddr;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        println!("begin to set RUST_LOG");
        std::env::set_var("RUST_LOG", "warn,rust_demo=info,rust_demo_core=info")
    }
    println!("RUST_LOG = {:?}", std::env::var_os("RUST_LOG"));
    let (non_blocking, _guard) =
        tracing_appender::non_blocking(rolling::daily("./logs", "app.log"));
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(false)
        .with_line_number(true)
        .with_timer(LogTimer)
        .with_writer(non_blocking)
        .init();
    info!("app server start");
    orm::init_app_conn().await;
    run_server().await;
}

async fn defalut_route<B>(req: Request<B>) -> impl IntoResponse {
    let method = req.method().as_str();
    let url = req.uri().path();
    Json(RspResult::fail(
        "没有找到匹配的URL".to_owned(),
        format!("{} : {}", method, url),
    ))
}

async fn run_server() {
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

    let arr: Vec<&str> = APP_CFG.server.ip.split(".").collect();
    let a = arr[0].parse::<u8>().unwrap();
    let b = arr[1].parse::<u8>().unwrap();
    let c = arr[2].parse::<u8>().unwrap();
    let d = arr[3].parse::<u8>().unwrap();
    let port: u16 = APP_CFG.server.port.try_into().unwrap();
    let addr = SocketAddr::from(([a, b, c, d], port));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
