use sea_orm::entity::prelude::*;
use sea_orm::FromQueryResult;
use sea_orm::NotSet;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Default, FromQueryResult, Deserialize, Serialize)]
pub struct CarVO {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub egin: Option<String>,
    pub page_num: Option<i32>,
    pub page_size: Option<i32>,
}
impl CarVO {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: if let Some(x) = self.id {
                Set(x)
            } else {
                NotSet
            },
            egin: if let Some(x) = self.egin {
                Set(x)
            } else {
                NotSet
            },
            user_id: if let Some(x) = self.user_id {
                Set(x)
            } else {
                NotSet
            },
        }
    }
}

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, DeriveActiveModelBehavior, Serialize, Deserialize,
)]
#[sea_orm(table_name = "car")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub egin: String,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
