pub mod car;
pub mod user;

use crate::util::db_utils;
use sea_orm::DatabaseConnection;

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
