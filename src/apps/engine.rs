use super::core::day;
use super::core::schedule;
use super::core::schedule::Schedule;
use super::core::task;

use async_std::fs::File;
use async_std::path::Path;
use async_std::prelude::*;
use serde_json::Result;
use std::env;

const SCHEDULE_FILE_PATH: &str = if cfg!(test) {
    "assets/tests/schedule.json"
} else {
    "assets/schedule.json"
};

const NOTIFICATION_CHECK_INTERVAL: u16 = 60;

fn read_project_root_path() -> String {
    if let Ok(project_root_path) = env::var("PROJECT_ROOT") {
        println!("Project root is set to: {}", project_root_path);
        project_root_path
    } else {
        println!("Project root is not set");
        "".to_string()
    }
}

#[derive(Debug)]
pub struct Engine {
    schedule: schedule::Schedule,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            schedule: schedule::Schedule::new(),
        }
    }

    pub async fn run(self: &mut Engine) -> std::io::Result<()> {
        println!("Engine is running");

        self.load_schedule().await?;

        // loop {
        //     self.check_notifications();

        //     async_std::task::sleep(std::time::Duration::from_secs(
        //         NOTIFICATION_CHECK_INTERVAL.into(),
        //     ))
        //     .await;
        // }

        Ok(())
    }

    pub async fn shutdown(self: &Engine) {
        println!("Engine is shutting down");
    }

    pub fn open_schedule_viewer(self: &Engine) {
        println!("Setting window is opened");
    }

    pub fn check_notifications(self: &Engine) {
        let days = self.schedule.get_days();

        for day in days {
            for task in day.get_tasks() {
                if task.get_start_time() != "2021-01-01 00:00:00" {
                    continue;
                }
                self.notify_task("Task is starting soon");
            }
        }
    }

    async fn load_schedule(self: &mut Engine) -> std::io::Result<()> {
        let project_root_path = read_project_root_path();

        let file_path = Path::new(&project_root_path).join(SCHEDULE_FILE_PATH);
        let mut file = File::open(&file_path).await?;

        let mut contents = String::new();

        file.read_to_string(&mut contents).await?;

        let schedule: schedule::Schedule = serde_json::from_str(&contents)?;
        self.schedule = schedule;

        Ok(())
    }

    fn notify_task(self: &Engine, message: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_engine() -> std::io::Result<()> {
        let mut engine = Engine::new();

        engine.run().await?;

        assert_ne!(engine.schedule.get_days().len(), 0);

        assert_eq!(engine.schedule.get_days()[0].get_day_of_week(), "Monday");
        assert_eq!(engine.schedule.get_days()[1].get_day_of_week(), "Tuesday");
        assert_eq!(engine.schedule.get_days()[2].get_day_of_week(), "Wednesday");
        assert_eq!(engine.schedule.get_days()[3].get_day_of_week(), "Thursday");
        assert_eq!(engine.schedule.get_days()[4].get_day_of_week(), "Friday");
        assert_eq!(engine.schedule.get_days()[5].get_day_of_week(), "Saturday");
        assert_eq!(engine.schedule.get_days()[6].get_day_of_week(), "Sunday");

        assert_eq!(
            engine.schedule.get_days()[0].get_tasks()[0].get_title(),
            "Team Meeting"
        );

        Ok(())
    }
}
