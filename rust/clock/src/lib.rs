use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let minutes_in_day = 24 * 60;

        let mut normalized = total_minutes % minutes_in_day;
        if normalized < 0 {
            normalized += minutes_in_day;
        }

        Clock {
            hours: normalized / 60,
            minutes: normalized % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.hours * 60 + self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
