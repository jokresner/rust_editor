use std::io::{stdout, Write};

use crossterm::terminal::enable_raw_mode;

use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: Result<(), std::io::Error>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = crossterm::terminal::size().unwrap();
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: enable_raw_mode(),
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!(
            "{}",
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        );
    }

    pub fn clear_current_line() {
        print!(
            "{}",
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)
        );
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(0);
        y = y.saturating_add(0);
        let x = x as u16;
        let y = y as u16;
        print!("{}", crossterm::cursor::MoveTo(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn cursor_hide() {
        print!("{}", crossterm::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", crossterm::cursor::Show);
    }

    pub fn set_bg_color(color: crossterm::style::Color) {
        print!("{}", crossterm::style::SetBackgroundColor(color));
    }

    pub fn set_fg_color(color: crossterm::style::Color) {
        print!("{}", crossterm::style::SetForegroundColor(color));
    }

    pub fn reset_colors() {
        print!("{}", crossterm::style::ResetColor);
    }
}
