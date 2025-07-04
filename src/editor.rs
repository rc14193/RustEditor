use crate::editor::terminal::{Position, Terminal};
use crossterm::event::{read, Event::{self, Key}, KeyCode::{self, Char}, KeyEvent, KeyModifiers, KeyEventKind};
mod terminal;

pub struct Editor {
    should_quit: bool,
    cursor_pos: Position
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false, cursor_pos: Position {x: 0, y: 0} }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;
        for row in 0..size.height {
            Terminal::move_cursor_to(Position { x: 0, y: row })?;
            Terminal::clear_line()?;
            if(row == size.height/3) {
                Terminal::display_welcome()?;
            } else {
                Terminal::print("~")?;
            }
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evealuate_event(&event)?;
            Terminal::move_cursor_to(self.cursor_pos)?;
            Terminal::execute()?;
        }
        Ok(())
    }

    fn evealuate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind,
            ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up if kind.is_release() => {
                    self.cursor_pos.y = self.cursor_pos.y.saturating_add_signed(-1);
                },                
                KeyCode::Down if kind.is_release() => {
                    self.cursor_pos.y = self.cursor_pos.y.saturating_add_signed(1);
                },                
                KeyCode::Left if kind.is_release() => {
                    self.cursor_pos.x = self.cursor_pos.x.saturating_add_signed(-1);
                },                
                KeyCode::Right if kind.is_release() => {
                    self.cursor_pos.x = self.cursor_pos.x.saturating_add_signed(1);
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(self.cursor_pos)?;
        }
        Terminal::show_cursor()?;
        Ok(())
    }
}
