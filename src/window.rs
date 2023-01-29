use std::io::{Stdout, stdout};
use crossterm::{ExecutableCommand, Command, Result, terminal};

pub(crate) struct Window {
    size:   (u16, u16),
    stdout: Stdout,
}
impl Window {
    pub fn init() -> Self {
        Self {
            size:   terminal::size().unwrap(),
            stdout: stdout(),
        }
    }
}
impl ExecutableCommand for Window {
    fn execute(&mut self, command: impl Command) -> Result<&mut Self> {
        self.stdout.execute(command)?;
        Ok(self)
    }
}