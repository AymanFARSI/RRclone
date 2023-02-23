mod lib {
    pub(crate) mod app;
    pub mod config;
}

mod ui {
    pub mod drive_ui;
    pub mod main_ui;
    pub mod settings_ui;
    pub mod tui;
}
use ui::tui::tui::start_tui;

fn main() {
    let _ = start_tui();
}
