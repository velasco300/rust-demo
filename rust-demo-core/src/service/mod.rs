pub mod user_service;

use crate::orm::APP_CONN;
use async_trait::async_trait;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait, IntoActiveModel, Value};
use std::str::FromStr;

#[async_trait]
pub trait BaseEntityTrait {
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> + ActiveModelBehavior + Send + Sync;
    type Entity: EntityTrait;

    async fn insert(
        user: Self::ActiveModel,
    ) -> Result<<<Self as BaseEntityTrait>::Entity as EntityTrait>::Model, anyhow::Error>
    where
        <<Self as BaseEntityTrait>::Entity as EntityTrait>::Model:
            IntoActiveModel<Self::ActiveModel>,
    {
        let conn = unsafe { APP_CONN.unwrap() };
        Ok(user.insert(conn).await?)
    }

    async fn delete(id: i32) -> Result<(), anyhow::Error>
    where
        <<<Self as BaseEntityTrait>::Entity as EntityTrait>::Column as FromStr>::Err:
            std::error::Error + Send + Sync,
    {
        let mut am = Self::ActiveModel::default();
        let c = <<Self as BaseEntityTrait>::Entity as EntityTrait>::Column::from_str("id")?;
        am.set(c, Value::Int(Some(id)));
        let conn = unsafe { APP_CONN.unwrap() };
        am.delete(conn).await?;
        Ok(())
    }
}
