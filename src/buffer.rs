use std::{
    collections::VecDeque,
    io::{Error, ErrorKind},
};
use crossterm::{Result, Command};

pub(crate) struct Buffer(
    VecDeque<Line>
);
struct Line {
    indent:  u16,
    content: String,
}
pub(crate) enum BufWriter {
    Insert(char),
    Delete,
    Enter,
}

impl Buffer {
    pub fn init() -> Self {
        Self(VecDeque::new())
    }

    fn line(&self, index: usize) -> Result<&Line> {
        Ok(self.0.get(index)
            .ok_or_else(||
                Error::new(ErrorKind::OutOfMemory, "Can't get line")
            )?
        )
    }
    fn line_mut(&mut self, index: usize) -> Result<&mut Line> {
        Ok(self.0.get_mut(index)
            .ok_or_else(||
                Error::new(ErrorKind::OutOfMemory, "Can't get line")
            )?
        )
    }

    pub fn insert(&mut self, (x, y): (usize, usize), char: char) -> Result<BufWriter> {
        let line = self.line_mut(y)?;
        line.content.insert(x, char);
        if char == ' ' && (x as u16) <= line.indent {line.indent += 1}
        Ok(BufWriter::Insert(char))
    }
    pub fn delete(&mut self, (x, y): (usize, usize)) -> Result<BufWriter> {
        self.line_mut(y)?.content.remove(x);
        Ok(BufWriter::Delete)
    }
    pub fn enter(&mut self, (x, y): (usize, usize)) -> Result<BufWriter> {
        let current_line = self.line_mut(y)?;
        let next_line = Line {
            indent: current_line.indent,
            content:
                " ".repeat(current_line.indent as usize)
                + &current_line.content.split_off(x),
        };

        self.0.insert(y + 1, next_line);
        Ok(BufWriter::Enter)
    }
}
impl Command for BufWriter {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        match self {
            Self::Insert(ch) => f.write_char(*ch),
            Self::Enter => f.write_str("\r\n"),
            Self::Delete => {

                f.write_str(r"\xb[")
            },
        }
    }
}