use std::fmt;

pub struct Slope {
    pub right: usize,
    pub down: usize,
}

impl fmt::Display for Slope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(right {}, down {})", self.right, self.down)
    }
}
