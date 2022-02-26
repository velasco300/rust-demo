use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn create_connection() -> Result<DatabaseConnection, DbErr> {
    Database::connect("mysql://root:@localhost/zzz").await
}
