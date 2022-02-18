use sea_schema::migration::{sea_query::*, *};
use rust_demo_core::orm::car;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220216_091122_add_user_id_to_cars"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(car::Entity)
                    .add_column(ColumnDef::new(car::Column::UserId).integer().default(1))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(car::Entity)
                    .drop_column(car::Column::UserId)
                    .to_owned(),
            )
            .await
    }
}
