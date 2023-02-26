#![allow(unused_doc_comments)]

pub mod app_mod {
    use std::{
        io::{self, Stdout},
        process::Child,
        thread::JoinHandle,
        time::Duration,
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
        },
        ui::{
            drive_ui::drive_ui::drive_ui, error_ui::error_ui::error_ui, main_ui::main_ui::main_ui,
            settings_ui::settings_ui::settings_ui,
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
        pub bottom_message: &'a str,
        pub mounted_drive: Option<DriveStruct>,
        pub rclone_process: Option<Child>,
        pub rclone_thread: Option<JoinHandle<()>>,
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
                bottom_message: "Use Arrow keys to navigate drives and press Enter",
                mounted_drive: None,
                rclone_process: None,
                rclone_thread: None,
            };
            app.drives.state.select(Some(0));
            app
        }

        pub fn start(&mut self) -> io::Result<()> {
            loop {
                match self.ui_idx {
                    0 => self.go_main(),
                    1 => self.go_drives(),
                    2 => self.go_settings(),
                    3 => {}
                    _ => panic!("Screen not found"),
                };
                if poll(Duration::from_millis(500))? {
                    match event::read().unwrap() {
                        Event::Resize(width, height) => {
                            if width < 80 || height < 21 {
                                self.error_temp_idx = self.ui_idx;
                                self.ui_idx = 3;
                                self.go_error(width, height);
                            } else {
                                let temp = self.error_temp_idx;
                                self.ui_idx = temp;
                                self.error_temp_idx = 0;
                            }
                        }
                        Event::Key(key) => {
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
                                    KeyCode::Char('q') => return Ok(()),
                                    KeyCode::Char('m') => self.ui_idx = 0,
                                    KeyCode::Char('d') => self.ui_idx = 1,
                                    KeyCode::Char('s') => self.ui_idx = 2,
                                    KeyCode::Down => self.drives.next(),
                                    KeyCode::Up => self.drives.previous(),
                                    KeyCode::Enter => {
                                        let i = self.drives.state.selected().unwrap();
                                        let mounted = self.drives.items[i].clone();
                                        match &self.mounted_drive {
                                            Some(drive) => {
                                                if drive.name.eq(&mounted.name) {
                                                    self.bottom_message =
                                                        "No need to re-mount same drive ^_^";
                                                } else {
                                                    stop_mounting(
                                                        &drive,
                                                        self.rclone_process.as_mut().unwrap(),
                                                        // self.rclone_thread.as_mut().unwrap(),
                                                    );
                                                    self.bottom_message= "Use Arrow keys to navigate drives and press Enter";
                                                    self.mounted_drive = Some(mounted.clone());
                                                    let ret = start_mounting(&mounted);
                                                    self.rclone_thread = Some(ret.0);
                                                    self.rclone_process = Some(ret.1);
                                                }
                                            }
                                            None => {
                                                self.bottom_message= "Use Arrow keys to navigate drives and press Enter";
                                                self.mounted_drive = Some(mounted.clone());
                                                let ret = start_mounting(&mounted);
                                                self.rclone_thread = Some(ret.0);
                                                self.rclone_process = Some(ret.1);
                                            }
                                        }
                                    }
                                    KeyCode::Backspace => match &self.mounted_drive {
                                        Some(drive) => {
                                            self.bottom_message = "Unmounting ...";
                                            stop_mounting(
                                                &drive,
                                                self.rclone_process.as_mut().unwrap(),
                                                // self.rclone_thread.as_mut().unwrap(),
                                            );
                                            self.mounted_drive = None;
                                            self.rclone_process = None;
                                            self.rclone_thread = None;
                                        }
                                        None => {
                                            self.bottom_message = "There is no drive to unmount!"
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
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
                .draw(|f| main_ui(f, &self.drives, &self.bottom_message))
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

        pub fn go_error(&mut self, width: u16, height: u16) {
            self.terminal
                .draw(|f| error_ui(f, width, height))
                .expect("Couldnt navigate to settings screen");
        }
    }
}
