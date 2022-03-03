use crate::controller::make_result;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{extract::Query, Json};
use rust_demo_core::common::RspResult;
use rust_demo_core::orm::car;
use rust_demo_core::service::{car_service::CarEntityTrait, BaseEntityTrait};

pub async fn add(Json(vo): Json<car::CarVO>) -> impl IntoResponse {
    Json(make_result(
        car::Entity::insert(vo.into_active_model()).await,
    ))
}

pub async fn delete(Path(id): Path<i32>) -> impl IntoResponse {
    Json(make_result(car::Entity::delete(id).await))
}

pub async fn update(Json(vo): Json<car::CarVO>) -> impl IntoResponse {
    println!("{:?}", vo);
    Json(make_result(
        car::Entity::update(vo.into_active_model()).await,
    ))
}

pub async fn show(Path(id): Path<i32>) -> impl IntoResponse {
    Json(make_result(car::Entity::find_by_id(id).await))
}

pub async fn list_by_egin(Query(vo): Query<car::CarVO>) -> impl IntoResponse {
    let egin = if vo.egin.is_none() {
        return Json(RspResult::fail("egin的值为空".to_owned(), None));
    } else {
        vo.egin.unwrap()
    };

    Json(make_result(car::Entity::list_by_egin(egin).await))
}
