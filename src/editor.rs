use core::panic;
use std::io::{self, Write};

use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{enable_raw_mode, self, Clear},
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _raw = enable_raw_mode();

        loop {
            if let Err(err) = self.refresh_screen() {
                die(&err);
            }
            if self.should_quit {
                break;
            }
            if let Err(err) = self.process_keypress() {
                die(&err);
            }
        }
    }

    pub fn default() -> Self {
        Self {should_quit: false}
    }

    fn refresh_screen(&self) -> Result<(),std::io::Error>{
        Clear(terminal::ClearType::All);
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let key = read()?;
        match key {
            Event::Key(key) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('q') {
                    self.should_quit = true;
                } else if key.modifiers.intersects(KeyModifiers::all()) {
                } else {
                }
            },
            _ => (),
        }
        Ok(())
    }
}

fn die(err: &std::io::Error) {
    panic!("{}", err);
}
