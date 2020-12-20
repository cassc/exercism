use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock(i32);

const ONE_DAY:i32 = 60 * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m = (hours * 60 + minutes).rem_euclid(ONE_DAY);
        Self(m)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m   = self.0 + minutes;
        Self(m.rem_euclid(ONE_DAY))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h = (self.0 / 60).rem_euclid(24);
        let m = self.0.rem_euclid(60);
        write!(f, "{:02}:{:02}", h, m)
    }
}
