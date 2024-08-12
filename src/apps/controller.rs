use super::core::schedule;
use super::core::schedule::Schedule;
use super::core::task;

use async_std::fs::File;
use async_std::path::Path;
use async_std::prelude::*;
use async_std::sync::Mutex;
use chrono::Local;
use notify_rust::Notification;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const SCHEDULE_FILE_PATH: &str = if cfg!(test) {
    "assets/tests/schedule.json"
} else {
    "assets/schedule.json"
};

fn read_project_root_path() -> String {
    if let Ok(project_root_path) = env::var("PROJECT_ROOT") {
        if cfg!(debug_assertions) {
            dbg!(project_root_path.as_str());
        }
        project_root_path
    } else {
        if cfg!(debug_assertions) {
            dbg!("Project root is not set");
        }
        "".to_string()
    }
}

pub struct Application {
    schedule: schedule::Schedule,
    finished_task_map: std::collections::HashMap<usize, AtomicBool>,
    is_shutdown: AtomicBool,
}

impl Application {
    pub fn new() -> Application {
        Application {
            schedule: Schedule::new(),
            finished_task_map: std::collections::HashMap::new(),
            is_shutdown: AtomicBool::new(false),
        }
    }

    pub async fn load_schedule(self: &mut Application) -> std::io::Result<()> {
        let project_root_path = read_project_root_path();

        let file_path = Path::new(&project_root_path).join(SCHEDULE_FILE_PATH);
        let mut file = File::open(&file_path).await?;

        let mut contents = String::new();

        file.read_to_string(&mut contents).await?;

        self.schedule = serde_json::from_str(&contents)?;

        Ok(())
    }

    pub fn check_shutdown(self: &Application) -> bool {
        self.is_shutdown.load(Ordering::Relaxed)
    }

    pub fn shutdown(self: &mut Application) {
        self.is_shutdown.store(true, Ordering::Relaxed);
    }

    pub fn open_schedule_viewer(self: &Application) {
        if cfg!(debug_assertions) {
            dbg!("Setting window is opened");
        }
    }

    pub fn check_notifications(self: &mut Application) {
        let current_time = Local::now().time();

        for day in self.schedule.get_days() {
            for task in day.get_tasks() {
                if cfg!(debug_assertions) {
                    dbg!(task.get_title());
                }

                let task_memory_address = task.get_memory_address();

                if self.finished_task_map.contains_key(&task_memory_address) {
                    continue;
                }

                let (is_converted, task_start_time, task_end_time) = task.get_time_range();

                if !is_converted || current_time < task_start_time || current_time > task_end_time {
                    self.finished_task_map
                        .insert(task_memory_address, AtomicBool::new(true));
                    continue;
                }

                self.notify_task(task);
                self.finished_task_map
                    .insert(task_memory_address, AtomicBool::new(false));
            }
        }
    }

    fn notify_task(self: &Application, task: &task::Task) {
        if cfg!(debug_assertions) {
            dbg!(task.get_title());
        }

        #[cfg(target_os = "macos")]
        static SOUND_NAME: &str = "Submarine";

        #[cfg(all(unix, not(target_os = "macos")))]
        static SOUND_NAME: &str = "message-new-instant";

        #[cfg(target_os = "windows")]
        static SOUND_NAME: &str = "Mail";

        Notification::new()
            .summary(task.get_title())
            .body(task.get_details())
            .sound_name(SOUND_NAME)
            .show()
            .unwrap();
    }
}

pub trait AsyncLoopInterface {
    async fn async_loop(&self);
}

const NOTIFICATION_CHECK_INTERVAL: u16 = 1;

impl AsyncLoopInterface for Arc<Mutex<Application>> {
    async fn async_loop(&self) {
        loop {
            if self.lock().await.check_shutdown() {
                return;
            }

            async_std::task::sleep(std::time::Duration::from_millis(
                NOTIFICATION_CHECK_INTERVAL.into(),
            ))
            .await;

            self.lock().await.check_notifications();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_load_schedule() -> std::io::Result<()> {
        let mut engine = Application::new();

        engine.load_schedule().await?;

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

        engine.check_notifications();

        Ok(())
    }
}
