use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn get_connection() -> Result<DatabaseConnection, DbErr> {
    Database::connect("mysql://root:@localhost/zzz").await
}
