use crate::{
    orm::{car, APP_CONN},
    service::BaseEntityTrait,
};
use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder};
use tracing::debug;

impl BaseEntityTrait for car::Entity {
    type Entity = car::Entity;
    type ActiveModel = car::ActiveModel;
}
impl CarEntityTrait for car::Entity {}

#[async_trait]
pub trait CarEntityTrait {
    async fn list_by_egin(egin: String) -> Result<Vec<car::Model>, anyhow::Error> {
        debug!("egin = {}", egin);
        let conn = unsafe { APP_CONN.unwrap() };
        let arr = car::Entity::find()
            .filter(Condition::all().add(car::Column::Egin.eq(egin)))
            .order_by_desc(car::Column::Id)
            .all(conn)
            .await?;
        Ok(arr)
    }
}
