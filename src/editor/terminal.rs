use crossterm::cursor::{self, Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::{Write, stdout};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
pub struct Terminal {}
#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn display_welcome() -> Result<(), std::io::Error> {
        let size = Self::size()?;
        let pos = Position {
            x: 0,
            y: size.height / 3,
        };
        Self::move_cursor_to(pos)?;
        let msg = format!("{NAME} -- version {VERSION}");
        let msg_len = msg.len();
        let padding = ((size.width as usize).saturating_sub(msg_len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        let mut welcome_msg = format!("~{spaces}{msg}");
        welcome_msg.truncate(size.width as usize);
        Self::print(&welcome_msg)?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::display_welcome()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn move_cursor_to(pos: Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(pos.x, pos.y))?;
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let s = size()?;
        Ok(Size {
            width: s.0,
            height: s.1,
        })
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn print(c: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(c))
    }
    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
}
