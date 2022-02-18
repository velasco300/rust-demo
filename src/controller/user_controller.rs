use crate::controller::make_result;
use axum::extract::Path;
use axum::{extract::Query, Json};
use rust_demo_core::{
    common::{PageResult, RspResult},
    orm::user,
    service::user_service::UserEntityTrait,
};

pub async fn index(
    Query(vo): Query<user::UserVO>,
) -> Json<RspResult<Option<PageResult<Vec<user::Model>>>>> {
    let page_num = if let Some(x) = vo.page_num { x } else { 1 };
    let page_size = if let Some(x) = vo.page_size { x } else { 10 };
    Json(make_result(
        user::Entity::page_user(page_num, page_size).await,
    ))
}

pub async fn add(Json(vo): Json<user::UserVO>) -> Json<RspResult<Option<user::Model>>> {
    Json(make_result(
        user::Entity::save_user(vo.into_active_model()).await,
    ))
}

pub async fn delete(Path(id): Path<i32>) -> Json<RspResult<Option<()>>> {
    Json(make_result(user::Entity::delete_user(id).await))
}

pub async fn update(Json(vo): Json<user::UserVO>) -> Json<RspResult<Option<user::Model>>> {
    println!("{:?}", vo);
    Json(make_result(
        user::Entity::update_user(vo.into_active_model()).await,
    ))
}

pub async fn show(Path(id): Path<i32>) -> Json<RspResult<Option<Option<user::Model>>>> {
    Json(make_result(user::Entity::query_by_id(id).await))
}
