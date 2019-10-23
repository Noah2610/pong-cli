#[derive(Clone, Deserialize)]
pub enum BallSpawnDirectionX {
    Left,
    Right,
    #[cfg(feature = "random")]
    Random,
}

impl BallSpawnDirectionX {
    pub fn number(&self, num: f32) -> f32 {
        match &self {
            Self::Left => -num,
            Self::Right => num,
            #[cfg(feature = "random")]
            Self::Random => Self::sample().number(num),
        }
    }

    #[cfg(feature = "random")]
    fn sample() -> Self {
        use rand::Rng;
        const COUNT: u8 = 2;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, COUNT) {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!("Random value should never get here"),
        }
    }
}

#[derive(Clone, Deserialize)]
pub enum BallSpawnDirectionY {
    None,
    Up,
    Down,
    #[cfg(feature = "random")]
    RandomUpOrDown,
    #[cfg(feature = "random")]
    Random,
}

impl BallSpawnDirectionY {
    pub fn number(&self, num: f32) -> f32 {
        match &self {
            Self::None => 0.0,
            Self::Up => -num,
            Self::Down => num,
            #[cfg(feature = "random")]
            Self::RandomUpOrDown => Self::sample_up_or_down().number(num),
            #[cfg(feature = "random")]
            Self::Random => Self::sample().number(num),
        }
    }

    #[cfg(feature = "random")]
    fn sample() -> Self {
        use rand::Rng;
        const COUNT: u8 = 3;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, COUNT) {
            0 => Self::None,
            1 => Self::Up,
            2 => Self::Down,
            _ => panic!("Random value should never get here"),
        }
    }

    #[cfg(feature = "random")]
    fn sample_up_or_down() -> Self {
        use rand::Rng;
        const COUNT: u8 = 2;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, COUNT) {
            0 => Self::Up,
            1 => Self::Down,
            _ => panic!("Random value should never get here"),
        }
    }
}
