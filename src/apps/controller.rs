use super::core::schedule;
use super::core::schedule::Schedule;
use super::core::task;
use super::view::window;

use async_std::channel::{Receiver, Sender};
use async_std::fs::File;
use async_std::path::Path;
use async_std::prelude::*;
use async_std::sync::Mutex;
use chrono::{Datelike, Local};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use notify_rust::Notification;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{env, thread};

const SCHEDULE_FILE_PATH: &str = if cfg!(test) {
    "assets/tests/schedule.json"
} else {
    "assets/schedule.json"
};

const ICON_FILE_PATH: &str = "assets/icon.ico";

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

struct FileObserver {
    changed_sender: Sender<String>,
    pub changed_receiver: Receiver<String>,
    file_path: String,
}

pub struct Application {
    file_observer: FileObserver,
    schedule: Arc<Mutex<schedule::Schedule>>,
    finished_task_map: std::collections::HashMap<usize, AtomicBool>,
    is_shutdown: AtomicBool,
    is_opened_viewer: AtomicBool,
}

impl FileObserver {
    pub fn new() -> FileObserver {
        let (changed_sender, changed_receiver) = async_std::channel::bounded(1);

        FileObserver {
            changed_sender,
            changed_receiver,
            file_path: "".to_string(),
        }
    }

    pub fn set_file_path(&mut self, file_path: &str) {
        self.file_path = file_path.to_string();
    }

    pub fn get_file_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub async fn observe_file(&self) -> std::io::Result<()> {
        let file_path = Arc::new(std::path::PathBuf::from(&self.file_path));
        let file_path_buf = file_path.to_path_buf();
        let tx = self.changed_sender.clone();

        thread::spawn(move || {
            let mut watcher = RecommendedWatcher::new(
                move |res: notify::Result<Event>| {
                    if let Ok(event) = res {
                        if event.paths.contains(&file_path_buf) {
                            let _ = tx.try_send("".to_string());
                        }
                    }
                },
                Config::default(),
            )
            .unwrap();

            watcher
                .watch(&file_path, RecursiveMode::NonRecursive)
                .unwrap();

            loop {
                std::thread::park();
            }
        });

        Ok(())
    }
}

impl Application {
    pub fn new() -> Application {
        Application {
            file_observer: FileObserver::new(),
            schedule: Arc::new(Mutex::new(Schedule::new())),
            finished_task_map: std::collections::HashMap::new(),
            is_shutdown: AtomicBool::new(false),
            is_opened_viewer: AtomicBool::new(false),
        }
    }

    pub fn get_schedule(self: &Application) -> Arc<Mutex<Schedule>> {
        self.schedule.clone()
    }

    pub async fn load_schedule(self: &mut Application) -> std::io::Result<()> {
        let project_root_path = read_project_root_path();

        let file_path = Path::new(&project_root_path).join(SCHEDULE_FILE_PATH);
        self.file_observer
            .set_file_path(file_path.to_str().unwrap());
        let mut file = File::open(file_path).await?;

        let mut contents = String::new();

        file.read_to_string(&mut contents).await?;

        {
            let mut schedule = self.schedule.lock().await;
            *schedule = serde_json::from_str(&contents)?;
        }

        Ok(())
    }

    pub async fn start_observer(self: &mut Application) -> std::io::Result<()> {
        self.file_observer.observe_file().await
    }

    pub fn check_shutdown(self: &Application) -> bool {
        self.is_shutdown.load(Ordering::Relaxed)
    }

    pub fn shutdown(self: &mut Application) {
        self.is_shutdown.store(true, Ordering::Relaxed);
    }

    pub fn check_opened_viewer(self: &Application) -> bool {
        self.is_opened_viewer.load(Ordering::Relaxed)
    }

    pub fn open_viewer(self: &mut Application) {
        self.is_opened_viewer.store(true, Ordering::Relaxed);
    }

    pub fn close_viewer(self: &mut Application) {
        self.is_opened_viewer.store(false, Ordering::Relaxed);
    }

    pub fn get_icon_file_path(self: &Application) -> String {
        Path::new(&read_project_root_path())
            .join(ICON_FILE_PATH)
            .to_str()
            .expect("Failed to convert path to string")
            .to_string()
    }

    pub async fn update_contents(&mut self) {
        match async_std::future::timeout(
            std::time::Duration::from_millis(NOTIFICATION_CHECK_INTERVAL.into()),
            self.file_observer.changed_receiver.recv(),
        )
        .await
        {
            Ok(Ok(_)) => {
                if let Ok(contents) =
                    async_std::fs::read_to_string(self.file_observer.get_file_path()).await
                {
                    if cfg!(debug_assertions) {
                        dbg!(&contents);
                    }
                    let mut schedule = self.schedule.lock().await;
                    *schedule = match serde_json::from_str(&contents) {
                        Ok(contents) => contents,
                        _ => {
                            dbg!("Failed to parse schedule file");
                            Schedule::new()
                        }
                    };
                }
            }
            Ok(Err(_)) => {
                dbg!("Failed to receive file change event");
                std::process::exit(-1);
            }
            Err(_) => (),
        }
    }

    pub async fn check_notifications(self: &mut Application) {
        let (current_time, current_day_of_week_string) = {
            let current_chrono = Local::now();
            let current_time = current_chrono.time();
            let current_day_of_week = current_chrono.weekday();

            let current_day_of_week_string = match current_day_of_week {
                chrono::Weekday::Mon => String::from("Monday"),
                chrono::Weekday::Tue => String::from("Tuesday"),
                chrono::Weekday::Wed => String::from("Wednesday"),
                chrono::Weekday::Thu => String::from("Thursday"),
                chrono::Weekday::Fri => String::from("Friday"),
                chrono::Weekday::Sat => String::from("Saturday"),
                chrono::Weekday::Sun => String::from("Sunday"),
            };

            (current_time, current_day_of_week_string)
        };

        for day in self.schedule.lock().await.get_days() {
            if day.get_day_of_week() != current_day_of_week_string {
                if cfg!(debug_assertions) {
                    dbg!(day.get_day_of_week());
                    dbg!(current_day_of_week_string.as_str());
                }
                continue;
            }

            for task in day.get_tasks() {
                if cfg!(debug_assertions) {
                    dbg!(task.get_title());
                }

                let task_memory_address = task.get_memory_address();

                if self.finished_task_map.contains_key(&task_memory_address)
                    && self.finished_task_map[&task_memory_address].load(Ordering::Relaxed)
                {
                    continue;
                }

                let (is_converted, task_start_time, task_end_time) = task.get_time_range();

                if !is_converted || current_time < task_start_time {
                    self.finished_task_map
                        .insert(task_memory_address, AtomicBool::new(false));
                    continue;
                }

                self.finished_task_map
                    .insert(task_memory_address, AtomicBool::new(true));

                if current_time > task_end_time {
                    continue;
                }

                self.notify_task(task);
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
    async fn wait_for_open_viewer(&self);
}

const NOTIFICATION_CHECK_INTERVAL: u16 = 100;

impl AsyncLoopInterface for Arc<Mutex<Application>> {
    async fn async_loop(&self) {
        loop {
            if self.lock().await.check_shutdown() {
                return;
            }

            self.lock().await.update_contents().await;

            self.lock().await.check_notifications().await;
        }
    }

    async fn wait_for_open_viewer(&self) {
        loop {
            if self.lock().await.check_shutdown() {
                return;
            }
            if !(self.lock().await.check_opened_viewer()) {
                continue;
            }

            async_std::task::sleep(std::time::Duration::from_millis(
                NOTIFICATION_CHECK_INTERVAL.into(),
            ))
            .await;

            {
                static WINDOW_TITLE: &str = "weshce -- schedule viewer";

                let schedule_clone = {
                    self.lock()
                        .await
                        .get_schedule()
                        .clone()
                        .lock()
                        .await
                        .clone()
                };
                window::open_window(WINDOW_TITLE, schedule_clone);
            }

            self.lock().await.close_viewer();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_load_schedule() {
        let mut app = Application::new();

        app.load_schedule().await.unwrap();

        app.check_notifications().await;

        let schedule = app.get_schedule().lock().await.clone();

        assert_ne!(schedule.get_days().len(), 0);

        assert_eq!(schedule.get_days()[0].get_day_of_week(), "Monday");
        assert_eq!(schedule.get_days()[1].get_day_of_week(), "Tuesday");
        assert_eq!(schedule.get_days()[2].get_day_of_week(), "Wednesday");
        assert_eq!(schedule.get_days()[3].get_day_of_week(), "Thursday");
        assert_eq!(schedule.get_days()[4].get_day_of_week(), "Friday");
        assert_eq!(schedule.get_days()[5].get_day_of_week(), "Saturday");
        assert_eq!(schedule.get_days()[6].get_day_of_week(), "Sunday");

        assert_eq!(
            schedule.get_days()[0].get_tasks()[0].get_title(),
            "Team Meeting"
        );
    }
}
