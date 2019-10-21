use crate::geo::Rect;

use super::component_prelude::*;

/// Entities' `Position`s are _confined_ to an area with this component.
/// Their `Position` may never leave this confined area.
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Confined {
    pub rect: Rect,
}
impl Confined {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }
}

impl From<Rect> for Confined {
    fn from(rect: Rect) -> Self {
        Self { rect }
    }
}

impl From<&Rect> for Confined {
    fn from(rect: &Rect) -> Self {
        Self { rect: rect.clone() }
    }
}
