pub mod car_service;
pub mod user_service;

use crate::orm::APP_CONN;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, EntityTrait, IntoActiveModel, PrimaryKeyTrait, Value,
};
use std::str::FromStr;

#[async_trait]
pub trait BaseEntityTrait {
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> + ActiveModelBehavior + Send + Sync;
    type Entity: EntityTrait;

    async fn insert<A>(am: A) -> Result<<A::Entity as EntityTrait>::Model, anyhow::Error>
    where
        A: ActiveModelTrait + ActiveModelBehavior + Send,
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    {
        let conn = unsafe { APP_CONN.unwrap() };
        Ok(am.insert(conn).await?)
    }

    async fn delete(id: i32) -> Result<(), anyhow::Error>
    where
        <<Self::Entity as EntityTrait>::Column as FromStr>::Err: std::error::Error + Send + Sync,
    {
        let mut am = Self::ActiveModel::default();
        let c = <Self::Entity as EntityTrait>::Column::from_str("id")?;
        am.set(c, Value::Int(Some(id)));
        let conn = unsafe { APP_CONN.unwrap() };
        am.delete(conn).await?;
        Ok(())
    }

    async fn update(
        am: Self::ActiveModel,
    ) -> Result<<Self::Entity as EntityTrait>::Model, anyhow::Error>
    where
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self::ActiveModel>,
    {
        let conn = unsafe { APP_CONN.unwrap() };
        Ok(am.update(conn).await?)
    }

    async fn find_by_id(
        id: <<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<<Self::Entity as EntityTrait>::Model>, anyhow::Error> {
        let conn = unsafe { APP_CONN.unwrap() };
        Ok(Self::Entity::find_by_id(id).one(conn).await?)
    }
}
