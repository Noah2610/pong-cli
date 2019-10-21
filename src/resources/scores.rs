use std::fmt;

use crate::geo::Side;

#[derive(Default, Clone)]
pub struct Score(u32);

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Clone)]
pub struct Scores {
    left_paddle:  Score,
    right_paddle: Score,
}

impl Scores {
    pub fn increase_for(&mut self, paddle_side: &Side) {
        self.get_mut(paddle_side).0 += 1;
    }

    pub fn get(&self, paddle_side: &Side) -> &Score {
        match paddle_side {
            Side::Left => &self.left_paddle,
            Side::Right => &self.right_paddle,
            Side::Top | Side::Bottom => {
                panic!("Paddle sides can only be Left or Right (Scores::get)")
            }
        }
    }

    fn get_mut(&mut self, paddle_side: &Side) -> &mut Score {
        match paddle_side {
            Side::Left => &mut self.left_paddle,
            Side::Right => &mut self.right_paddle,
            Side::Top | Side::Bottom => {
                panic!("Paddle sides can only be Left or Right (Scores::get)")
            }
        }
    }
}
