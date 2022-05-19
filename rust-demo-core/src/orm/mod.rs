pub mod car;
pub mod user;

use crate::util::db_utils;
use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;

// use async_once::AsyncOnce;
// use lazy_static::lazy_static;
// lazy_static! {
//     pub static ref CONN: AsyncOnce<DatabaseConnection> = AsyncOnce::new(async {
//         let conn = db_utils::get_connection().await.unwrap();
//         conn
//     });
// }

pub static mut APP_CONN: Option<&DatabaseConnection> = None;

pub async fn init_app_conn() {
    unsafe {
        if APP_CONN.is_none() {
            let c = Box::new(db_utils::create_connection().await.unwrap());
            APP_CONN = Some(Box::leak(c));
        }
    }
}

// 另一种全局单例变量创建示例
pub static CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

async fn init_conn() -> DatabaseConnection {
    db_utils::create_connection().await.unwrap()
}

pub async fn get_conn() -> &'static DatabaseConnection {
    CONN.get_or_init(init_conn).await
}
