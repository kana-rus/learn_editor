use std::{
    io::{stdout, Stdout},
    time::Duration,
};
use crossterm::{
    Result,
    cursor,
    terminal, terminal::ClearType,
    style, style::Print,
    event, event::{
        Event,
        KeyCode,
        KeyModifiers as Modifier,
    }, Command, ExecutableCommand,
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

struct Buffer {
    stdout:  Stdout,
    content: Vec<String>,
}

struct Window {
    size: (u16, u16),
}

struct Cursor {
    position: (u16, u16),
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
            .execute(cursor::MoveTo(0, 0))?;
        terminal::enable_raw_mode()?;
        Ok(Self::default())
    }
    fn execute(&mut self, command: impl Command) -> Result<()> {
        self.buffer.stdout.execute(command)?;
        Ok(())
    }
    fn edit(&mut self) -> Result<()> {
        while event::poll(Duration::from_secs(60))? {
            if let Event::Key(e) = event::read()? {
                let input = (e.modifiers, e.code);

                if input == self.config.exit_key {break}
                match input {
                    (Modifier::NONE, KeyCode::Char(ch)) => self.execute(Print(ch))?,
                    (_, KeyCode::Enter) => self.execute(Print("\r\n"))?,
                    (_, KeyCode::Backspace | KeyCode::Delete) => self.execute(terminal::Clear(ClearType::CurrentLine))?,

                    (_, KeyCode::Up   ) => self.execute(cursor::MoveUp(1))?,
                    (_, KeyCode::Down ) => self.execute(cursor::MoveDown(1))?,
                    (_, KeyCode::Left ) => self.execute(cursor::MoveLeft(1))?,
                    (_, KeyCode::Right) => self.execute(cursor::MoveRight(1))?,

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
            buffer: Buffer {
                stdout:  stdout(),
                content: vec![],
            },
            cursor: Cursor {
                position: (0, 0),
            },
            window: Window {
                size: terminal::size().unwrap(),
            }
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
