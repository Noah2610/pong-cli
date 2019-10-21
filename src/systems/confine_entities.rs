use super::system_prelude::*;

/// This system confines all entities with `Transform` and `Confined`
/// to the rect defined in `Confined`, taking `Size` into account.
#[derive(Default)]
pub struct ConfineEntitiesSystem;

impl<'a> System<'a> for ConfineEntitiesSystem {
    type SystemData = (
        ReadStorage<'a, Confined>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (confineds, sizes, mut positions): Self::SystemData) {
        for (confined, size_opt, position) in
            (&confineds, sizes.maybe(), &mut positions).join()
        {
            if let Some(size) = size_opt {
                let half_size = (size.w * 0.5, size.h * 0.5);
                let rect = Rect {
                    top:    position.y - half_size.1,
                    bottom: position.y + half_size.1,
                    left:   position.x - half_size.0,
                    right:  position.x + half_size.0,
                };
                if rect.left < confined.rect.left {
                    position.x = confined.rect.left + half_size.0;
                }
                if rect.right > confined.rect.right {
                    position.x = confined.rect.right - half_size.0;
                }
                if rect.top < confined.rect.top {
                    position.y = confined.rect.top + half_size.1;
                }
                if rect.bottom > confined.rect.bottom {
                    position.y = confined.rect.bottom - half_size.1;
                }
            } else {
                position.x =
                    position.x.min(confined.rect.right).max(confined.rect.left);
                position.y =
                    position.y.min(confined.rect.bottom).max(confined.rect.top);
            }
        }
    }
}
