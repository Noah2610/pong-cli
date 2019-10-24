pub trait RoundDown
where
    Self: Into<f32>,
{
    /// Like `f32::round`, but rounds half-way cases TOWARDS `0.0`,
    /// and returns an `i32`.
    fn round_lower(self: Self) -> i32 {
        let n: f32 = self.into();
        let fract = n.fract();
        (if fract > 0.5 { n.ceil() } else { n.floor() }) as i32
    }

    /// Like `f32::round`, but returns `i32`.
    fn round_higher(self: Self) -> i32 {
        self.into().round() as i32
    }
}

impl RoundDown for f32 {
}
