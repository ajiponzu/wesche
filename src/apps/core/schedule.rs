use super::day;

#[derive(Debug)]
pub struct Schedule {
    days: Vec<day::Day>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule { days: Vec::new() }
    }

    pub fn get_days(self: &Schedule) -> &Vec<day::Day> {
        &self.days
    }

    pub fn add_day(self: &mut Schedule, day: day::Day) {
        self.days.push(day);
    }
}
