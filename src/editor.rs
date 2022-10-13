use core::panic;
use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{enable_raw_mode, Clear},
};

use crate::Terminal;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
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
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal.")
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_line();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\n");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            println!("~\r");
        }
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
            }
            _ => (),
        }
        Ok(())
    }
}

fn die(err: &std::io::Error) {
    print!("{}", Clear(crossterm::terminal::ClearType::All));
    panic!("{}", err);
}
