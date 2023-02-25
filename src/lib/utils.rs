#![allow(dead_code)]

pub mod utils {
    use log::{debug, error, info, trace, warn, LevelFilter};

    pub fn log_error(msg: String) {
        error!(target:"error", "{}", msg);
    }

    pub fn log_warning(msg: String) {
        warn!(target:"warn", "{}", msg);
    }

    pub fn log_trace(msg: String) {
        trace!(target:"trace", "{}", msg);
    }

    pub fn log_debug(msg: String) {
        debug!(target:"debug", "{}", msg);
    }

    pub fn log_info(msg: String) {
        info!(target:"info", "{}", msg);
    }

    pub fn get_levelfilter_emoji(level: LevelFilter) -> String {
        match level {
            LevelFilter::Error => String::from("😥"),
            LevelFilter::Warn => String::from("😁"),
            LevelFilter::Trace => String::from("😑"),
            LevelFilter::Debug => String::from("😌"),
            LevelFilter::Info => String::from("🫡"),
            _ => String::new(),
        }
    }
}
