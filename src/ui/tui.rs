pub mod tui {

    use std::io;

    use crossterm::event::{self, Event, KeyCode};

    use crate::lib::app::app_mod::App;

    pub fn start_tui() -> Result<(), io::Error> {
        let mut app: App = App::new();

        let res = run_app(&mut app);

        app.exit();

        if let Err(err) = res {
            println!("{:?}", err)
        }

        Ok(())
    }

    fn run_app(app: &mut App) -> io::Result<()> {
        loop {
            match app.ui_idx {
                0 => app.go_main(),
                1 => app.go_drives(),
                2 => app.go_settings(),
                _ => panic!("Screen not found"),
            };

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('m') => app.ui_idx = 0,
                    KeyCode::Char('d') => app.ui_idx = 1,
                    KeyCode::Char('s') => app.ui_idx = 2,
                    _ => {}
                }
            }
        }
    }
}
