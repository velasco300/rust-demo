pub mod db_utils;

use core::fmt;
use std::fs;
use time::{format_description, OffsetDateTime, UtcOffset};
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

use crate::common::etc::AppConfig;

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
