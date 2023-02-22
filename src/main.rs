mod lib {
    pub mod config;
}

use lib::config::config::read_config;

fn main() {
    let rclone_config = read_config();
    println!("{:#?}", rclone_config);
}
