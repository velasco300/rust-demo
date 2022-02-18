use sea_schema::migration::{sea_query::*, *};
use rust_demo_core::orm::user;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220213_102936_add_age_to_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(user::Entity)
                    .add_column(ColumnDef::new(user::Column::Age).integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(user::Entity)
                    .drop_column(user::Column::Age)
                    .to_owned(),
            )
            .await
    }
}
