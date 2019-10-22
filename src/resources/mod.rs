pub mod prelude {
    pub use crossterm::{AlternateScreen, TerminalCursor, TerminalInput};

    pub use super::deltatime::Deltatime;
    pub use super::running::Running;
    pub use super::scores::{Score, Scores};
    pub use super::should_reset::{ShouldReset, ShouldResetBallSpawns};
    pub use crate::input::prelude::*;
}

mod deltatime;
mod running;
mod scores;
mod should_reset;
