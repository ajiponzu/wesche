use super::day;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Schedule {
    days: Vec<day::Day>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule { days: Vec::new() }
    }

    pub fn get_days(&self) -> &Vec<day::Day> {
        self.days.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule() {
        let schedule = Schedule::new();

        assert_eq!(schedule.get_days().len(), 0);
    }
}
