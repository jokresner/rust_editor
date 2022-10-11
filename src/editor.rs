use core::panic;

use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::enable_raw_mode,
};

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _raw = enable_raw_mode();

        loop {
            if let Ok(key) = read() {
                match key {
                    Event::Key(key) => {
                        if KeyCode::Char('q') == key.code
                            && key.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            break;
                        } else if key.modifiers.intersects(KeyModifiers::all()) {
                            println!("{:?}\r", key.modifiers)
                        } else {
                            println!("{:?}\r", key.code)
                        }
                    }
                    _ => panic!("no matching key"),
                }
            }
        }
    }
}
