use std::time::Duration;
use crossterm::{
    terminal, Result,
    event, event::{
        Event, KeyCode, KeyEvent,
        KeyModifiers as Modifier,
    },
};

#[derive(Debug, PartialEq)]
struct Key {
    code:     KeyCode,
    modifier: Modifier,
}
impl From<KeyEvent> for Key {
    fn from(value: KeyEvent) -> Self {
        Self {
            code:     value.code,
            modifier: value.modifiers,
        }
    }
}

struct Config {
    exit_key: Key,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            exit_key: Key {
                code:     KeyCode::Char('q'),
                modifier: Modifier::CONTROL,
            },
        }
    }
}

struct Editor {
    config: Config,
}
impl Editor {
    fn new() -> Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(Self::default())
    }
    fn read_input(&mut self) -> Result<()> {
        while event::poll(Duration::from_secs(10))? {
            if let Event::Key(e) = event::read()? {
                let input = Key::from(e);
                if input == self.config.exit_key {return Ok(())}
                println!("{:?}\r", input)
            }
        }
        Ok(())
    }
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            config: Config::default(),
        }
    }
}
impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Counldn't exit raw mode");
    }
}

fn main() -> Result<()> {
    Editor::new()?.read_input()?;
    Ok(())
}
