mod lib {
    pub mod app;
    pub mod config;
    pub mod mount;
    pub mod utils;
}

mod ui {
    pub mod drive_ui;
    pub mod error_ui;
    pub mod main_ui;
    pub mod settings_ui;
}

use std::io;

use lib::app::app_mod::App;

fn main() -> Result<(), io::Error> {
    // match simple_logging::log_to_file("test.log", LevelFilter::Info) {
    //     Ok(_) => println!("start loggin"),
    //     Err(e) => println!("{}", e),
    // }

    let mut app: App = App::new();

    let res = app.start();

    app.exit();

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
