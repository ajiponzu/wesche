use super::task;

#[derive(Debug)]
pub struct Day {
    date: chrono::naive::NaiveDate,
    tasks: Vec<task::Task>,
}

impl Day {
    pub fn new(date: chrono::naive::NaiveDate) -> Day {
        Day {
            date,
            tasks: Vec::new(),
        }
    }

    pub fn get_date(self: &Day) -> String {
        self.date.to_string()
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
        let day = Day {
            date: chrono::naive::NaiveDate::parse_from_str("2021-01-01", "%Y-%m-%d").unwrap(),
            tasks: Vec::new(),
        };

        assert_eq!(day.get_date(), "2021-01-01");
    }
}
