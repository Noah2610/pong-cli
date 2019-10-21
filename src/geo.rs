pub mod prelude {
    pub use super::Side;
}

#[derive(Clone, Copy, PartialEq)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}
