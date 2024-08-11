use super::task;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Day {
    day_of_week: String,
    tasks: Vec<task::Task>,
}

impl Day {
    pub fn get_day_of_week(self: &Day) -> &str {
        self.day_of_week.as_str()
    }

    pub fn get_tasks(self: &Day) -> &Vec<task::Task> {
        self.tasks.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let day = Day {
            day_of_week: "Monday".to_string(),
            tasks: vec![],
        };

        assert_eq!(day.get_day_of_week(), "Monday");
    }
}
