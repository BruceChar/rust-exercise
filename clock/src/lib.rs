use std::string::ToString;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}



impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours,
            minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    fn normalize(&self) -> (i32, i32) {
        let mut dif_hours = self.minutes / 60;
        let mut minutes = self.minutes % 60;
        if minutes < 0 {
            dif_hours -= 1;
            minutes += 60;
        }
        let hours = ((self.hours + dif_hours) % 24 + 24) % 24;
        (hours, minutes)
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        let (hours, minutes) = self.normalize();
        format!("{:0>2}:{:0>2}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        let (first_hours, first_minutes) = self.normalize();
        let (second_hours, second_minutes) = other.normalize();
        (first_hours == second_hours) && (first_minutes == second_minutes)
    }
}

impl Eq for Clock {}