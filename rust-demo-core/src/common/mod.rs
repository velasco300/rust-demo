use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AppErr {
    pub msg: String,
}
impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppErr {}", self.msg)
    }
}
impl Error for AppErr {}
impl AppErr {
    pub fn throw_app_error(msg: String) -> Result<(), AppErr> {
        Err(AppErr { msg })
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PageResult<T> {
    pub page_num: i32,
    pub page_size: i32,
    pub pages: i32,
    pub date: T,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RspResult<T> {
    pub success: bool,
    pub code: String,
    pub msg: String,
    pub data: T,
}
impl<T> RspResult<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            code: "0x001".to_owned(),
            msg: "成功".to_owned(),
            data,
        }
    }
    pub fn fail(msg: String, data: T) -> Self {
        Self {
            success: false,
            code: "0x004".to_owned(),
            msg,
            data,
        }
    }
}

#[test]
fn test_myapp() {
    dbg!(RspResult {
        data: PageResult::<i32> {
            ..Default::default()
        },
        ..Default::default()
    });

    dbg!(RspResult::<Option<PageResult<i32>>> {
        ..Default::default()
    });
}
