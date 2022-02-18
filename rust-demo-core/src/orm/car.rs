use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, DeriveActiveModelBehavior, Serialize, Deserialize,
)]
#[sea_orm(table_name = "car")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub egin: String,
    pub user_id: i32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
