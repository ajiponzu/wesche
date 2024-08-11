use super::core::day;
use super::core::schedule;
use super::core::task;

const SCHEDULE_FILE_PATH: &str = "schedule.json";
const NOTIFICATION_CHECK_INTERVAL: u16 = 60;

pub struct Engine {
    schedule: schedule::Schedule,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            schedule: schedule::Schedule::new(),
        }
    }

    pub async fn run(self: &Engine) {
        println!("Engine is running");
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

    fn add_day(self: &mut Engine, day: day::Day) {
        self.schedule.add_day(day);
    }

    fn load_schedule(self: &mut Engine) {}

    fn notify_task(self: &Engine, message: &str) {}
}
