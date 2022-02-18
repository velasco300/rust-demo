use sea_schema::migration::{sea_query::*, *};
use rust_demo_core::orm::car;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220213_183618_create_cars"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(car::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(car::Column::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(car::Column::Egin).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(car::Entity).to_owned())
            .await
    }
}
