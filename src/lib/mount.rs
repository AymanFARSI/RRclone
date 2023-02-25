#![allow(dead_code)]

pub mod mount {

    use log::LevelFilter;

    use crate::lib::{config::config::DriveStruct, utils::utils::*};

    pub fn start_mounting(drive: &DriveStruct) {
        log_info(format!(" Start mounting drive \"{}\"", drive.name));
    }

    pub fn stop_mounting(drive: &DriveStruct) {
        log_warning(format!(
            " Stop mounting drive \"{}\"\nSave your stuff {}",
            drive.name,
            get_levelfilter_emoji(LevelFilter::Warn)
        ));
    }
}
