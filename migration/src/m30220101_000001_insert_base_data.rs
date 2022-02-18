use rust_demo_core::orm::{car, user};
use sea_orm::Set;
use sea_schema::migration::{
    sea_orm::{DeleteMany, EntityTrait, Insert, QueryTrait},
    *,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m30220101_000001_insert_base_data"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut users = Vec::new();
        let mut cars = Vec::new();
        (1..=10).for_each(|i| {
            /*
            let u = user::Model {
                id: i,
                age: Some(10 + i),
                user_name: format!("tony{}", i),
            };
            // arr.push(<user::Model as IntoActiveModel<user::ActiveModel>>::into_active_model(u)); or
            // arr.push(IntoActiveModel::into_active_model(u)); or
            arr.push(u.into_active_model());
            */
            let uam = user::ActiveModel {
                // id: sea_orm::NotSet,
                // age: Set(None),
                age: Set(10 + i),
                user_name: Set(format!("tony{}", i)),
                ..Default::default()
            };
            users.push(uam);

            let cam = car::ActiveModel {
                egin: Set("lbjn".to_owned()),
                user_id: Set(3),
                ..Default::default()
            };
            cars.push(cam);
        });

        manager
            .exec_stmt(<Insert<user::ActiveModel> as QueryTrait>::into_query(
                user::Entity::insert_many(users),
            ))
            .await?;

        manager
            .exec_stmt(<Insert<car::ActiveModel> as QueryTrait>::into_query(
                car::Entity::insert_many(cars),
            ))
            .await

        // let u = user::Model {
        //     id: 1,
        //     age: 18,
        //     user_name: String::from("tony"),
        // };
        // manager
        //     .exec_stmt(<Insert<user::ActiveModel> as QueryTrait>::into_query(
        //         user::Entity::insert(
        //             <user::Model as IntoActiveModel<user::ActiveModel>>::into_active_model(u),
        //         ),
        //     ))
        //     .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(<DeleteMany<car::Entity> as QueryTrait>::into_query(
                car::Entity::delete_many(),
            ))
            .await?;

        manager
            .exec_stmt(<DeleteMany<user::Entity> as QueryTrait>::into_query(
                user::Entity::delete_many(),
            ))
            .await
    }
}
