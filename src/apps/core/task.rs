use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let task = Task {
            title: "Test Task".to_string(),
            start_time: "2021-01-01 00:00:00".to_string(),
            end_time: "2021-01-01 01:00:00".to_string(),
            details: "This is a test task".to_string(),
        };

        assert_eq!(task.get_title(), "Test Task");
        assert_eq!(task.get_start_time(), "2021-01-01 00:00:00");
        assert_eq!(task.get_end_time(), "2021-01-01 01:00:00");
        assert_eq!(task.get_details(), "This is a test task");
    }
}
