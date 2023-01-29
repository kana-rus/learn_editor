use crossterm::Command;

pub(crate) struct Cursor {
    position: (usize, usize),
}
impl Cursor {
    pub fn init() -> Self {
        Self {
            position: (0, 0),
        }
    }

    pub fn move_x(&mut self, len: i32) -> &mut Self {
        if let Some(new_x) = (self.position.0 as i32).checked_add(len) {
            self.position.0 = new_x as usize
        }
        self
    }
    pub fn move_y(&mut self, len: i32) -> &mut Self {
        if let Some(new_y) = (self.position.1 as i32).checked_add(len) {
            self.position.1 = new_y as usize
        }
        self
    }
    pub fn set_x(&mut self, new_x: usize) -> &mut Self {
        self.position.0 = new_x;
        self
    }
    pub fn set_y(&mut self, new_y: usize) -> &mut Self {
        self.position.1 = new_y;
        self
    }
}
impl Command for &mut Cursor {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        crossterm::cursor::MoveTo(self.position.0 as u16, self.position.1 as u16).write_ansi(f)
    }
}