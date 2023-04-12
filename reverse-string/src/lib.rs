pub fn reverse(input: &str) -> String {
    let mut s = input.chars().collect::<Vec<_>>();
    let len = s.len();
    for i in 0..(len / 2) {
        let tmp = s[i];
        s[i] = s[len - i - 1];
        s[len - i - 1] = tmp;
    }

    String::from_iter(s)
}

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
        let (minutes, mut hours) = (self.minutes + minutes, self.hours);
        let (dif_hours, minutes) = (minutes / 60, minutes % 60);
        hours = (self.hours + dif_hours) % 24;
        Clock::new(hours, minutes)
    } 
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        let (mut first, mut second) = (self.add_minutes(0), other.add_minutes(0));
        first.hours = (first.hours % 24 + 24) % 24;
        second.hours = (second.hours % 24 + 24) % 24;
        (first.hours == second.hours) && (first.minutes == second.minutes)
    }
}

impl Eq for Clock {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_neg_hours() {
        assert_eq!(Clock::new(-26, 1).add_minutes(121), Clock::new(-24, 1).add_minutes(1));
    }

    #[test]
    fn minutes_roll_over() {
        assert_eq!(Clock::new(0, 160), Clock::new(2, 40));
    }
}
