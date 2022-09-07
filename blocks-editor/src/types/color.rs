use std::fmt;
use serde::Serialize;
#[derive(Copy, Clone, Serialize)]
pub struct Color {
    pub(crate) c1:i32,
    pub(crate) c2:i32,
    pub(crate) c3:i32,
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Color:")
            .field("c1", &self.c1)
            .field("c2", &self.c2)
            .field("c3", &self.c3)
            .finish()
    }
}


impl Default for Color {
    fn default() -> Self {
        Self  { c1:0,c2:0,c3:0 }
    }
}
