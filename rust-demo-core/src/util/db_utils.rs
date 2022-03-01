use crate::util::APP_CFG;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn create_connection() -> Result<DatabaseConnection, DbErr> {
    Database::connect(APP_CFG.database.url.as_str()).await
}
