mod buffer; use buffer::Buffer;
mod cursor; use cursor::Cursor;
mod window; use window::Window;

use std::{
    io::stdout,
    time::Duration,
};
use crossterm::{
    Result,
    ExecutableCommand,
    style::Print,
    terminal, terminal::ClearType,
    event, event::{
        Event,
        KeyCode,
        KeyModifiers as Modifier,
    },
};


struct Config {
    exit_key: (Modifier, KeyCode),
}
impl Default for Config {
    fn default() -> Self {
        Self {
            exit_key: (Modifier::CONTROL, KeyCode::Char('q')),
        }
    }
}

struct Editor {
    config: Config,
    buffer: Buffer,
    cursor: Cursor,
    window: Window,
}
impl Editor {
    fn new() -> Result<Self> {
        stdout()
            .execute(terminal::Clear(ClearType::All))?
            .execute(crossterm::cursor::MoveTo(0, 0))?;
        terminal::enable_raw_mode()?;
        Ok(Self::default())
    }
    fn edit(&mut self) -> Result<()> {
        while event::poll(Duration::from_secs(60))? {
            if let Event::Key(e) = event::read()? {
                let input = (e.modifiers, e.code);

                if input == self.config.exit_key {break}
                match input {
                    (Modifier::NONE, KeyCode::Char(ch)) => {
                        self.window
                            .execute(Print(ch))?
                            .execute(self.cursor.move_x(1))?;
                    },
                    (_, KeyCode::Enter) => {
                        self.window
                            .execute(Print("\r\n"))?
                            .execute(self.cursor.set_x(0).move_y(1))?;
                    },
                    (_, KeyCode::Backspace | KeyCode::Delete) => {
                        self.window
                            .execute(terminal::Clear(ClearType::CurrentLine))?
                            .execute(self.cursor.move_x(-1))?;
                    },

                    (_, KeyCode::Up   ) => {self.window.execute(self.cursor.move_y(1))?;},
                    (_, KeyCode::Down ) => {self.window.execute(self.cursor.move_y(-1))?;},
                    (_, KeyCode::Left ) => {self.window.execute(self.cursor.move_x(-1))?;},
                    (_, KeyCode::Right) => {self.window.execute(self.cursor.move_x(1))?;},

                    _ => (),
                }
            }
        }
        Ok(())
    }
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            config: Config::default(),
            buffer: Buffer::init(),
            cursor: Cursor::init(),
            window: Window::init(),
        }
    }
}
impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Counldn't exit raw mode");
    }
}

fn main() -> Result<()> {
    Editor::new()?.edit()?;
    Ok(())
}
