use crossterm::{
    event::{
        Event::{self, Key},
        KeyCode::Char,
        KeyEvent, KeyModifiers, read,
    },
    style::Print,
};
use std::io::Error;

mod terminal;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VER: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialise().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            // kind,
            // state,
            .. // ignores rest
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => {}
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Print("Until next time\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VER}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}\r\n");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for row in 0..height - 1 {
            Terminal::clear_line()?;
            if row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Terminal::print("~\r\n")?;
            }
        }
        Print("~");
        Ok(())
    }
}
