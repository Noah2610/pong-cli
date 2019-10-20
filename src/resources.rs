pub mod prelude {
    pub use super::Running;
    pub use crossterm::AlternateScreen;
    pub use crossterm::TerminalCursor;
}

pub struct Running(pub bool);
