use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::system_prelude::*;
use ball_storages::BallStorages;

#[derive(Default)]
pub struct SpawnBallSystem {
    balls_spawned_at: HashMap<Index, Instant>,
    last_spawn_at:    Option<Instant>,
}

impl<'a> System<'a> for SpawnBallSystem {
    type SystemData =
        (Entities<'a>, ReadExpect<'a, Settings>, BallStorages<'a>);

    fn run(
        &mut self,
        (entities, settings, mut ball_storages): Self::SystemData,
    ) {
        let now = Instant::now();
        let spawn_delay_duration =
            Duration::from_millis(settings.ball.spawn_delay_ms);
        let spawn_next_ball_duration =
            Duration::from_millis(settings.ball.spawn_next_ball_in_ms);
        let target_balls_amount = settings.ball.balls_amount;
        let current_balls_amount = ball_storages.balls.join().count() as u16;
        let initial_ball_velocity = settings.ball.velocity;

        // Make previously spawned, non-moving balls move
        let mut remove_balls_spawned_at = Vec::new();
        for (id, spawned_at) in &self.balls_spawned_at {
            if now.duration_since(spawned_at.clone()) >= spawn_delay_duration {
                (
                    &entities,
                    &ball_storages.balls,
                    &mut ball_storages.velocities,
                )
                    .join()
                    .find(|(ball_entity, _, _)| ball_entity.id() == *id)
                    .map(|(_, _, ball_velocity)| {
                        // TODO: In which direction should balls begin to move?
                        ball_velocity.x = initial_ball_velocity.0;
                        ball_velocity.y = initial_ball_velocity.1;
                    });
                remove_balls_spawned_at.push(*id);
            }
        }
        for id_to_remove in remove_balls_spawned_at {
            self.balls_spawned_at.remove(&id_to_remove);
        }

        // Spawn new, non-moving balls
        if current_balls_amount < target_balls_amount {
            if let Some(last_spawn_at) = self.last_spawn_at {
                if now.duration_since(last_spawn_at) >= spawn_next_ball_duration
                {
                    self.spawn_ball(&entities, &settings, &mut ball_storages);
                }
            } else {
                self.spawn_ball(&entities, &settings, &mut ball_storages);
            }
        }
    }
}

impl SpawnBallSystem {
    fn spawn_ball(
        &mut self,
        entities: &Entities,
        settings: &Settings,
        BallStorages {
            balls,
            positions,
            sizes,
            velocities,
            drawables,
            collisions,
            colliders,
        }: &mut BallStorages,
    ) {
        let now = Instant::now();

        let ball_entity = entities
            .build_entity()
            .with(Ball::default(), balls)
            .with(Drawable::new(settings.chars.ball), drawables)
            .with(
                Position::new(
                    settings.room.width as f32 * 0.5,
                    settings.room.height as f32 * 0.5,
                ),
                positions,
            )
            .with(Size::new(settings.ball.size.0, settings.ball.size.1), sizes)
            .with(Velocity::default(), velocities)
            .with(Collision::new(CollisionType::Ball), collisions)
            .with(Collider::default(), colliders)
            .build();

        self.balls_spawned_at.insert(ball_entity.id(), now);

        self.last_spawn_at = Some(now);
    }
}

mod ball_storages {
    use super::*;
    use specs::{prelude::ResourceId, SystemData};

    #[derive(SystemData)]
    pub struct BallStorages<'a> {
        pub balls:      WriteStorage<'a, Ball>,
        pub positions:  WriteStorage<'a, Position>,
        pub sizes:      WriteStorage<'a, Size>,
        pub velocities: WriteStorage<'a, Velocity>,
        pub drawables:  WriteStorage<'a, Drawable>,
        pub collisions: WriteStorage<'a, Collision>,
        pub colliders:  WriteStorage<'a, Collider>,
    }
}