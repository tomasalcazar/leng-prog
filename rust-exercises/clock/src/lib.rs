use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    // Create a new Clock instance, normalizing hours and minutes
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        Clock::from_minutes(total_minutes)
    }

    // Helper function to create a Clock from total minutes, rolling over as needed
    fn from_minutes(minutes: i32) -> Self {
        let mut total_minutes = minutes % (24 * 60); // Total minutes in a day (1440)
        if total_minutes < 0 {
            total_minutes += 24 * 60; // Adjust for negative minutes
        }
        Clock {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }

    // Adds or subtracts minutes, creating a new Clock
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::from_minutes(self.hours * 60 + self.minutes + minutes)
    }
}

// Implement the Display trait for Clock to format as "HH:MM"
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
