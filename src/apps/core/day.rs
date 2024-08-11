use super::task;

#[derive(Debug, Clone, PartialEq)] // Add the PartialEq trait
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug)]
pub struct Day {
    day_of_week: DayOfWeek,
    tasks: Vec<task::Task>,
}

impl Day {
    pub fn new(day_of_week: &str) -> Day {
        Day {
            day_of_week: match day_of_week {
                "Sunday" => DayOfWeek::Sunday,
                "Monday" => DayOfWeek::Monday,
                "Tuesday" => DayOfWeek::Tuesday,
                "Wednesday" => DayOfWeek::Wednesday,
                "Thursday" => DayOfWeek::Thursday,
                "Friday" => DayOfWeek::Friday,
                "Saturday" => DayOfWeek::Saturday,
                _ => DayOfWeek::Sunday,
            },
            tasks: Vec::new(),
        }
    }

    pub fn get_day_of_week(self: &Day) -> Option<DayOfWeek> {
        Some(self.day_of_week.clone())
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

        assert_eq!(day.get_day_of_week().unwrap(), DayOfWeek::Monday);
    }
}
