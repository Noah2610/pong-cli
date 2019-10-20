pub mod prelude {
    pub use super::Running;
    pub use crossterm::{AlternateScreen, TerminalCursor, TerminalInput};
}

pub struct Running(pub bool);
