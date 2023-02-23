pub mod app_mod {
    use std::io::{self, Stdout};

    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use tui::{backend::CrosstermBackend, Terminal};

    use crate::ui::{
        drive_ui::drive_ui::drive_ui, main_ui::main_ui::main_ui,
        settings_ui::settings_ui::settings_ui,
    };

    pub struct App {
        pub ui_idx: u8,
        pub terminal: Terminal<CrosstermBackend<Stdout>>,
    }

    impl App {
        pub fn new() -> App {
            enable_raw_mode().expect("couldnt enable raw mode");
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
                .expect("error executing EnterAlternateScreen & EnableMouseCapture");
            let backend = CrosstermBackend::new(stdout);
            let terminal = Terminal::new(backend).expect("couldnt create terminal backend");
            App {
                ui_idx: 0,
                terminal,
            }
        }

        pub fn exit(&mut self) {
            disable_raw_mode().expect("couldnt disable raw mode");
            execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )
            .expect("error executing LeaveAlternateScreen & DisableMouseCapture");
            self.terminal.show_cursor().expect("couldnt show cursor");
        }

        pub fn go_main(&mut self) {
            self.terminal
                .draw(main_ui)
                .expect("Couldnt navigate to main screen");
        }

        pub fn go_drives(&mut self) {
            self.terminal
                .draw(drive_ui)
                .expect("Couldnt navigate to drive screen");
        }

        pub fn go_settings(&mut self) {
            self.terminal
                .draw(settings_ui)
                .expect("Couldnt navigate to settings screen");
        }
    }
}
