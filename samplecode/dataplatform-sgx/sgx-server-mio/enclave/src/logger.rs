use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use std::io::Write;
use std::string::String;

pub fn init_env_log(level_str: String) {
    let level: LevelFilter = match level_str.to_ascii_lowercase().as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Off,
    };

    Builder::new()
        .filter(None, level)
        .format(|buf, record| {
            writeln!(buf, "{} {} [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
