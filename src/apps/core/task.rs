use chrono::NaiveTime;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Task {
    title: String,
    start_time: String,
    end_time: String,
    details: String,
}

impl Task {
    pub fn get_title(self: &Task) -> &str {
        self.title.as_str()
    }

    pub fn get_start_time(self: &Task) -> &str {
        self.start_time.as_str()
    }

    pub fn get_end_time(self: &Task) -> &str {
        self.end_time.as_str()
    }

    pub fn get_details(self: &Task) -> &str {
        self.details.as_str()
    }

    pub fn get_time_range(self: &Task) -> (bool, NaiveTime, NaiveTime) {
        let (is_converted_start, start_time) = Task::convert_string_to_time(self.get_start_time());
        let (is_converted_end, end_time) = Task::convert_string_to_time(self.get_end_time());

        (is_converted_start && is_converted_end, start_time, end_time)
    }

    pub fn get_memory_address(self: &Task) -> usize {
        self as *const _ as usize
    }

    fn convert_string_to_time(time_str: &str) -> (bool, NaiveTime) {
        match NaiveTime::parse_from_str(time_str, "%H:%M:%S") {
            Ok(time_result) => (true, time_result),
            Err(_) => (false, NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let task = Task {
            title: "Test Task".to_string(),
            start_time: "00:00:00".to_string(),
            end_time: "01:00:00".to_string(),
            details: "This is a test task".to_string(),
        };

        assert_eq!(task.get_title(), "Test Task");
        assert_eq!(task.get_start_time(), "00:00:00");
        assert_eq!(task.get_end_time(), "01:00:00");
        assert_eq!(task.get_details(), "This is a test task");
    }
}
