pub mod prelude {
    pub use crossterm::{AlternateScreen, TerminalCursor, TerminalInput};

    pub use super::deltatime::Deltatime;
    pub use super::running::Running;
    pub use super::scores::{Score, Scores};
    pub use crate::input::prelude::*;
}

mod deltatime;
mod running;
mod scores;
