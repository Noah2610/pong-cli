use std::time::{Duration, Instant};

pub struct Deltatime {
    last_update:           Instant,
    duration_since_update: Option<Duration>,
}

impl Deltatime {
    pub fn update(&mut self) {
        let now = Instant::now();

        self.duration_since_update = Some(now.duration_since(self.last_update));

        self.last_update = now;
    }

    pub fn delta_seconds(&self) -> f32 {
        if let Some(duration_since_update) = self.duration_since_update.as_ref()
        {
            duration_since_update.as_secs_f32()
        } else {
            0.0
        }
    }
}

impl Default for Deltatime {
    fn default() -> Self {
        Self {
            last_update:           Instant::now(),
            duration_since_update: None,
        }
    }
}
