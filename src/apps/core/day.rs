use super::task;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Day {
    day_of_week: String,
    tasks: Vec<task::Task>,
}

impl Day {
    pub fn new(day_of_week: &str) -> Day {
        Day {
            day_of_week: day_of_week.to_string(),
            tasks: Vec::new(),
        }
    }

    pub fn get_day_of_week(self: &Day) -> &str {
        &self.day_of_week.as_str()
    }

    pub fn get_tasks(self: &Day) -> &Vec<task::Task> {
        &self.tasks
    }

    pub fn add_task(self: &mut Day, task: task::Task) {
        self.tasks.push(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let day = Day::new("Monday");

        assert_eq!(day.get_day_of_week(), "Monday");
    }
}
