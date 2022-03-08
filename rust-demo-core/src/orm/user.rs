use sea_orm::{entity::prelude::*, FromQueryResult, NotSet, Set};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Default, FromQueryResult, Deserialize, Serialize)]
pub struct UserDTO {
    pub id: i32,
    pub age: i32,
    pub user_name: String,
    pub car_id: i32,
    pub egin: String,
}

#[derive(Debug, Default, FromQueryResult, Deserialize, Serialize)]
pub struct UserVO {
    pub id: Option<i32>,
    pub age: Option<i32>,
    pub user_name: Option<String>,
    pub car_id: Option<i32>,
    pub egin: Option<String>,
    pub page_num: Option<i32>,
    pub page_size: Option<i32>,
}
impl UserVO {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: if let Some(x) = self.id {
                Set(x)
            } else {
                NotSet
            },
            age: if let Some(x) = self.age {
                Set(x)
            } else {
                NotSet
            },
            user_name: if let Some(x) = self.user_name {
                Set(x.clone())
            } else {
                NotSet
            },
        }
    }
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id)
    }
}

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, DeriveActiveModelBehavior, Serialize, Deserialize,
)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_name: String,
    pub age: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::car::Entity")]
    Car,
}

impl Related<super::car::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Car.def()
    }
}
