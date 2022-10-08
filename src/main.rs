use std::io::{stdin, Read};

use crossterm::terminal::{enable_raw_mode, is_raw_mode_enabled};

fn main() {
    if is_raw_mode_enabled().is_err() {
        let raw = enable_raw_mode();
        if raw.is_err() {
            return;
        }
    }

    for b in stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?}\r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => panic!("{}", err),
        }
    }
}

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}
