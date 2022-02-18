use crate::common::PageResult;
use crate::orm::car;
use crate::{orm::user::*, util::db_utils};
use async_trait::async_trait;
use sea_orm::ActiveModelTrait;
use sea_orm::QueryOrder;
use sea_orm::Set;
use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait};
use sea_orm::{Condition, EntityTrait, JoinType};

impl UserEntityTrait for Entity {}

#[async_trait]
pub trait UserEntityTrait {
    async fn save_user(user: ActiveModel) -> Result<Model, anyhow::Error> {
        Ok(user.insert(&db_utils::get_connection().await?).await?)
    }

    async fn save_many_user(users: Vec<ActiveModel>) -> Result<(), anyhow::Error> {
        Entity::insert_many(users)
            .exec(&db_utils::get_connection().await?)
            .await?;
        Ok(())
    }

    async fn delete_user(id: i32) -> Result<(), anyhow::Error> {
        ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db_utils::get_connection().await?)
        .await?;
        Ok(())
    }

    async fn update_user(user: ActiveModel) -> Result<Model, anyhow::Error> {
        Ok(user.update(&db_utils::get_connection().await?).await?)
    }

    async fn query_by_id(id: i32) -> Result<Option<Model>, anyhow::Error> {
        /* example option to result
        Entity::find_by_id(id)
            .one(&db_utils::get_connection().await?)
            .await?
            .ok_or(anyhow!("没有查询到数据"))
        */
        Ok(Entity::find_by_id(id)
            .one(&db_utils::get_connection().await?)
            .await?)
    }

    async fn page_user(
        page_num: i32,
        page_size: i32,
    ) -> Result<PageResult<Vec<Model>>, anyhow::Error> {
        let mut page_num = page_num;
        let mut page_size = page_size;
        if page_num < 1 {
            page_num = 1;
        }
        if page_size < 1 {
            page_size = 1;
        }
        let conn = db_utils::get_connection().await?;
        let paginator = Entity::find()
            .order_by_desc(Column::Id)
            .paginate(&conn, page_size.try_into()?);
        let pages = paginator.num_pages().await?;
        let arr = paginator.fetch_page((page_num - 1).try_into()?).await?;

        Ok(PageResult {
            page_num,
            page_size,
            pages: pages.try_into()?,
            date: arr,
        })
    }

    async fn list_by_age_and_egin(age: i32, egin: String) -> Result<Vec<UserVO>, anyhow::Error> {
        let arr = Entity::find()
            .select_only()
            .column(Column::Id)
            .column(Column::UserName)
            .column(Column::Age)
            .column(car::Column::Egin)
            .column_as(car::Column::Id, "car_id")
            .join(JoinType::InnerJoin, Relation::Car.def())
            .filter(
                Condition::all()
                    .add(Column::Age.gt(age))
                    .add(car::Column::Egin.eq(egin)),
            )
            .order_by_desc(Column::Id)
            .limit(10)
            .offset(1)
            .into_model::<UserVO>()
            .all(&db_utils::get_connection().await?)
            .await?;
        Ok(arr)
    }
}

#[test]
fn test_gefdfd() {
    // let s = Entity::find()
    //     .select_only()
    //     .column(Column::Id)
    //     .column(Column::UserName)
    //     .column(Column::Age)
    //     .column(car::Column::Egin)
    //     .column_as(car::Column::Id, "car_id")
    //     .join(JoinType::InnerJoin, Relation::Car.def())
    //     .filter(
    //         Condition::all()
    //             .add(Column::Age.gt(18))
    //             .add(car::Column::Egin.eq("lbjn")),
    //     )
    //     .order_by_desc(Column::Id)
    //     .limit(3)
    //     .offset(2)
    //     .build(sea_orm::DbBackend::MySql)
    //     .to_string();
    // println!("-----{}", s);
}