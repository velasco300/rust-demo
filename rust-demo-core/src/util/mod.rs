pub mod db_utils;

use crate::common::etc::AppConfig;
use core::fmt;
use lazy_static::lazy_static;
use std::fs;
use time::{format_description, OffsetDateTime, UtcOffset};
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

lazy_static! {
    pub static ref APP_CFG: AppConfig = read_config();
}

#[derive(Debug)]
pub struct LogTimer;

impl FormatTime for LogTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        let dt = OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(8, 0, 0).unwrap());
        let fm =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]")
                .unwrap();
        write!(w, "{}", dt.format(&fm).unwrap())
    }
}

pub fn read_config() -> AppConfig {
    let s = fs::read_to_string("config/application.toml").unwrap();
    let cfg: AppConfig = toml_edit::easy::from_str(&s).unwrap();
    cfg
}
