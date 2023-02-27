#![allow(unused_doc_comments)]

pub mod app_mod {
    use std::{
        io::{self, Stdout},
        process::Child,
        time::Duration,
        vec,
    };

    use log::LevelFilter;

    use crossterm::{
        event::{
            self, poll, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture,
            EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, Event, KeyCode,
            KeyEventKind, KeyModifiers, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
            PushKeyboardEnhancementFlags,
        },
        execute, queue,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use tui::{backend::CrosstermBackend, widgets::ListState, Terminal};
    use tui_logger::{init_logger, set_default_level};

    use crate::{
        lib::{
            config::config::{read_rclone_config, ConfigStruct, DriveStruct},
            mount::mount::{start_mounting, stop_mounting},
            utils::utils::login_google_drive,
        },
        ui::{
            drive_ui::drive_ui::drive_ui, error_ui::error_ui::error_ui, main_ui::main_ui::main_ui,
        },
    };

    pub struct StatefulList<DriveStruct> {
        pub state: ListState,
        pub items: Vec<DriveStruct>,
    }

    impl<DriveStruct: std::clone::Clone> StatefulList<DriveStruct> {
        pub fn with_items(items: &Vec<DriveStruct>) -> StatefulList<DriveStruct> {
            StatefulList {
                state: ListState::default(),
                items: items.to_vec(),
            }
        }

        pub fn next(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }

        pub fn previous(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }

    pub struct App<'a> {
        pub ui_idx: u8,
        pub error_temp_idx: u8,
        pub terminal: Terminal<CrosstermBackend<Stdout>>,
        pub rclone_conf: ConfigStruct,
        pub drives: StatefulList<DriveStruct>,
        pub main_message: &'a str,
        pub drive_message: &'a str,
        pub drives_mounted: Vec<DriveStruct>,
        pub processes_mounted: Vec<Child>,
        pub insert_mode: bool,
        pub new_name: String,
    }

    impl App<'_> {
        pub fn new() -> App<'static> {
            enable_raw_mode().expect("couldnt enable raw mode");
            let mut stdout = io::stdout();
            if matches!(
                crossterm::terminal::supports_keyboard_enhancement(),
                Ok(true)
            ) {
                queue!(
                    stdout,
                    PushKeyboardEnhancementFlags(
                        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                            | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                            | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                    )
                )
                .expect("error start queueing");
            }
            execute!(
                stdout,
                EnterAlternateScreen,
                EnableMouseCapture,
                EnableBracketedPaste,
                EnableFocusChange
            )
            .expect("error starting execution");
            let backend = CrosstermBackend::new(stdout);
            let terminal = Terminal::new(backend).expect("couldnt create terminal backend");

            let rclone_conf = read_rclone_config();

            init_logger(LevelFilter::Trace).unwrap();
            set_default_level(LevelFilter::Trace);

            let mut app = App {
                ui_idx: 0,
                error_temp_idx: 0,
                terminal,
                rclone_conf: rclone_conf.clone(),
                drives: StatefulList::with_items(&rclone_conf.drives),
                main_message: "Use Arrow keys to navigate drives and press Enter",
                drive_message: "Managing drives",
                drives_mounted: vec![],
                processes_mounted: vec![],
                insert_mode: false,
                new_name: String::new(),
            };
            app.drives.state.select(Some(0));
            app
        }

        pub fn start(&mut self) -> io::Result<()> {
            loop {
                match self.ui_idx {
                    0 => self.go_main(),
                    1 => self.go_drives(),
                    2 => {}
                    _ => panic!("Screen not found"),
                };
                if poll(Duration::from_millis(500))? {
                    match event::read().unwrap() {
                        Event::Resize(width, height) => {
                            if width < 80 || height < 21 {
                                self.error_temp_idx = self.ui_idx;
                                self.ui_idx = 2;
                                self.go_error(width, height);
                            } else {
                                let temp = self.error_temp_idx;
                                self.ui_idx = temp;
                                self.error_temp_idx = 0;
                            }
                        }
                        Event::Key(key) => match &self.ui_idx {
                            0 => {
                                if key.kind == KeyEventKind::Press {
                                    match key.modifiers {
                                        KeyModifiers::SHIFT => {
                                            if key.code == KeyCode::Right {
                                                println!("SHIFT + Right");
                                            }
                                        }
                                        _ => {}
                                    }
                                    match key.code {
                                        KeyCode::Char('q') => {
                                            for index in 0..self.processes_mounted.len() {
                                                stop_mounting(
                                                    &self.drives_mounted[index],
                                                    &mut self.processes_mounted[index],
                                                );
                                            }
                                            return Ok(());
                                        }
                                        KeyCode::Char('d') => self.ui_idx = 1,
                                        KeyCode::Char('r') => {
                                            self.drive_message = "Refreshing list of drives";
                                            let rclone_conf = read_rclone_config();
                                            self.rclone_conf = rclone_conf.clone();
                                            self.drives =
                                                StatefulList::with_items(&rclone_conf.drives);
                                            self.drives.state.select(Some(0));
                                        }
                                        KeyCode::Down => self.drives.next(),
                                        KeyCode::Up => self.drives.previous(),
                                        KeyCode::Enter => {
                                            let i = self.drives.state.selected().unwrap();
                                            let mounted = self.drives.items[i].clone();
                                            if self.drives_mounted.contains(&mounted) {
                                                self.main_message =
                                                    "No need to re-mount same drive ^_^";
                                            } else {
                                                self.main_message = "Mounting ...";
                                                self.drives_mounted.push(mounted.clone());
                                                start_mounting(&mounted, self);
                                            }
                                        }
                                        KeyCode::Delete => {
                                            let i = self.drives.state.selected().unwrap();
                                            let mounted = self.drives.items[i].clone();
                                            if self.drives_mounted.contains(&mounted) {
                                                self.main_message = "Unmounting ...";
                                                let i = self
                                                    .drives_mounted
                                                    .iter()
                                                    .position(|x| x == &mounted)
                                                    .unwrap();
                                                stop_mounting(
                                                    &mounted,
                                                    &mut self.processes_mounted[i],
                                                );
                                                self.drives_mounted.remove(i);
                                                self.processes_mounted.remove(i);
                                            } else {
                                                self.main_message = "There is no drive to unmount!"
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            1 => {
                                if key.kind == KeyEventKind::Press {
                                    match key.code {
                                        KeyCode::Esc => self.insert_mode = false,
                                        KeyCode::Down => self.drives.next(),
                                        KeyCode::Up => self.drives.previous(),
                                        KeyCode::Enter => {
                                            if self.insert_mode {
                                                self.drive_message =
                                                    "Exit insert mode first 'Esc'";
                                            } else {
                                                self.drive_message =
                                                    "After login, press 'r' to refresh";
                                                self.insert_mode = false;
                                                login_google_drive(self.new_name.clone());
                                            }
                                        }
                                        KeyCode::Backspace => {
                                            self.new_name.pop();
                                        }
                                        KeyCode::Char(c) => {
                                            if self.insert_mode {
                                                self.new_name.push(c);
                                            } else {
                                                match c {
                                                    'i' => self.insert_mode = true,
                                                    'm' => self.ui_idx = 0,
                                                    'q' => {
                                                        for index in 0..self.processes_mounted.len()
                                                        {
                                                            stop_mounting(
                                                                &self.drives_mounted[index],
                                                                &mut self.processes_mounted[index],
                                                            );
                                                        }
                                                        return Ok(());
                                                    }
                                                    'r' => {
                                                        self.drive_message =
                                                            "Refreshing list of drives";
                                                        let rclone_conf = read_rclone_config();
                                                        self.rclone_conf = rclone_conf.clone();
                                                        self.drives = StatefulList::with_items(
                                                            &rclone_conf.drives,
                                                        );
                                                        self.drives.state.select(Some(0));
                                                    }
                                                    // 'e' => {
                                                    //     self.drive_message = "Editing ...";
                                                    // }
                                                    _ => {}
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }

        pub fn exit(&mut self) {
            disable_raw_mode().expect("couldnt disable raw mode");
            if matches!(
                crossterm::terminal::supports_keyboard_enhancement(),
                Ok(true)
            ) {
                queue!(self.terminal.backend_mut(), PopKeyboardEnhancementFlags,)
                    .expect("error exit queueing");
            }
            execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture,
                DisableBracketedPaste,
                DisableFocusChange,
            )
            .expect("error executing execution");
            self.terminal.show_cursor().expect("couldnt show cursor");
        }

        pub fn go_main(&mut self) {
            self.terminal
                .draw(|f| main_ui(f, &self.drives, &self.main_message))
                .expect("Couldnt navigate to main screen");
        }

        pub fn go_drives(&mut self) {
            self.terminal
                .draw(|f| {
                    drive_ui(
                        f,
                        &self.drives,
                        &self.drive_message,
                        self.new_name.clone(),
                        self.insert_mode.clone(),
                    )
                })
                .expect("Couldnt navigate to drive screen");
        }

        pub fn go_error(&mut self, width: u16, height: u16) {
            self.terminal
                .draw(|f| error_ui(f, width, height))
                .expect("Couldnt navigate to error screen");
        }
    }
}
