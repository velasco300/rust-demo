use crate::controller::make_result;
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use rust_demo_core::{
    common::{PageResult, RspResult},
    orm::user,
    service::user_service::UserEntityTrait,
};
use tracing::{debug, error, info, warn};

pub async fn index(
    Query(vo): Query<user::UserVO>,
) -> Json<RspResult<Option<PageResult<Vec<user::Model>>>>> {
    debug!("age = {:?}", vo.age);
    info!("user_name = {:?}", vo.user_name);
    warn!("page_num = {:?}", vo.page_num);
    error!("page_size = {:?}", vo.page_size);
    let page_num = vo.page_num.unwrap_or(1);
    let page_size = vo.page_size.unwrap_or(10);
    Json(make_result(
        user::Entity::page_user(page_num, page_size).await,
    ))
}

// pub async fn add(Json(vo): Json<user::UserVO>) -> Json<RspResult<Option<user::Model>>> {
pub async fn add(Json(vo): Json<user::UserVO>) -> impl IntoResponse {
    Json(make_result(
        user::Entity::save_user(vo.into_active_model()).await,
    ))
}

// pub async fn delete(Path(id): Path<i32>) -> Json<RspResult<Option<()>>> {
pub async fn delete(Path(id): Path<i32>) -> impl IntoResponse {
    Json(make_result(user::Entity::delete_user(id).await))
}

// pub async fn update(Json(vo): Json<user::UserVO>) -> Json<RspResult<Option<user::Model>>> {
pub async fn update(Json(vo): Json<user::UserVO>) -> impl IntoResponse {
    println!("{:?}", vo);
    Json(make_result(
        user::Entity::update_user(vo.into_active_model()).await,
    ))
}

// pub async fn show(Path(id): Path<i32>) -> Json<RspResult<Option<Option<user::Model>>>> {
pub async fn show(Path(id): Path<i32>) -> impl IntoResponse {
    Json(make_result(user::Entity::query_by_id(id).await))
}

pub async fn pg_by_age_and_egin(Query(vo): Query<user::UserVO>) -> impl IntoResponse {
    let age = if vo.age.is_none() {
        return Json(RspResult::fail("age的值为空".to_owned(), None));
    } else {
        vo.age.unwrap()
    };
    debug!("{:?}", age);
    let egin = if vo.egin.is_none() {
        return Json(RspResult::fail("egin的值为空".to_owned(), None));
    } else {
        vo.egin.unwrap()
    };

    Json(make_result(
        user::Entity::list_by_age_and_egin(age, egin).await,
    ))
}
