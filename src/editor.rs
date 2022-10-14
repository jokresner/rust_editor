use core::panic;
use std::env;

use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::Clear,
};

use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
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
            terminal: Terminal::default().expect("Failed to initialize terminal."),
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_line();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\n");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn move_cursor(&mut self, key: KeyCode) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height,
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y };
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Rustitor editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let key = read()?;
        match key {
            Event::Key(key) => {
                if key.modifiers.is_empty() {
                    match key.code {
                        KeyCode::Left
                        | KeyCode::Right
                        | KeyCode::Up
                        | KeyCode::Down
                        | KeyCode::PageDown
                        | KeyCode::PageUp
                        | KeyCode::End
                        | KeyCode::Home => {
                            self.move_cursor(key.code);
                        }
                        _ => (),
                    }
                } else {
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('q')
                    {
                        self.should_quit = true;
                    }
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
