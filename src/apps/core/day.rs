use super::task;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Day {
    day_of_week: String,
    tasks: Vec<task::Task>,
}

impl Day {
    pub fn get_day_of_week(&self) -> &str {
        self.day_of_week.as_str()
    }

    pub fn get_tasks(&self) -> &Vec<task::Task> {
        self.tasks.as_ref()
    }

    pub fn compare_day_of_week(&self, week_day: &chrono::Weekday) -> bool {
        match week_day {
            chrono::Weekday::Mon => self.day_of_week == "Monday" || self.day_of_week == "月曜日",
            chrono::Weekday::Tue => self.day_of_week == "Tuesday" || self.day_of_week == "火曜日",
            chrono::Weekday::Wed => self.day_of_week == "Wednesday" || self.day_of_week == "水曜日",
            chrono::Weekday::Thu => self.day_of_week == "Thursday" || self.day_of_week == "木曜日",
            chrono::Weekday::Fri => self.day_of_week == "Friday" || self.day_of_week == "金曜日",
            chrono::Weekday::Sat => self.day_of_week == "Saturday" || self.day_of_week == "土曜日",
            chrono::Weekday::Sun => self.day_of_week == "Sunday" || self.day_of_week == "日曜日",
        }
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

        assert_eq!(day.get_day_of_week(), "Moday");
    }
}
