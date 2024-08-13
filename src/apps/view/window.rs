use super::super::core::day::Day;
use super::super::core::schedule::Schedule;
use super::super::core::task::Task;

use druid::widget::{Flex, Label, List};
use druid::{im, AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

#[derive(Data, Clone, Lens)]
struct TaskForDruid {
    title: String,
    start_time: String,
    end_time: String,
    details: String,
}

#[derive(Data, Clone, Lens)]
struct DayForDruid {
    day_of_week: String,
    tasks: im::Vector<TaskForDruid>,
}

#[derive(Data, Clone, Lens)]
struct ScheduleForDruid {
    days: im::Vector<DayForDruid>,
}

impl TaskForDruid {
    fn from(task: &Task) -> TaskForDruid {
        TaskForDruid {
            title: task.get_title().to_string(),
            start_time: task.get_start_time().to_string(),
            end_time: task.get_end_time().to_string(),
            details: task.get_details().to_string(),
        }
    }
}

impl DayForDruid {
    fn from(day: &Day) -> DayForDruid {
        DayForDruid {
            day_of_week: day.get_day_of_week().to_string(),
            tasks: day.get_tasks().iter().map(TaskForDruid::from).collect(),
        }
    }
}

impl ScheduleForDruid {
    fn from(schedule: &Schedule) -> ScheduleForDruid {
        ScheduleForDruid {
            days: schedule.get_days().iter().map(DayForDruid::from).collect(),
        }
    }
}

pub fn open_window(window_title: &str, window_size: (f64, f64), schedule: Schedule) {
    let main_window = WindowDesc::new(build_ui())
        .title(window_title)
        .window_size(window_size);

    let schedule_for_druid = ScheduleForDruid::from(&schedule);

    AppLauncher::with_window(main_window)
        .launch(schedule_for_druid)
        .expect("Failed to launch application");
}

fn build_ui() -> impl Widget<ScheduleForDruid> {
    List::new(|| {
        Flex::column()
            .with_child(
                Label::new(|day: &DayForDruid, _env: &_| format!("Day: {}", day.day_of_week))
                    .padding(5.0),
            )
            .with_child(
                List::new(|| {
                    // For each task, create a label with task details
                    Flex::column()
                        .with_child(
                            Label::new(|task: &TaskForDruid, _env: &_| {
                                format!("Title: {}", task.title)
                            })
                            .padding(5.0),
                        )
                        .with_child(
                            Label::new(|task: &TaskForDruid, _env: &_| {
                                format!("Time: {} - {}", task.start_time, task.end_time)
                            })
                            .padding(5.0),
                        )
                        .with_child(
                            Label::new(|task: &TaskForDruid, _env: &_| {
                                format!("Details: {}", task.details)
                            })
                            .padding(5.0),
                        )
                })
                .lens(DayForDruid::tasks),
            )
    })
    .lens(ScheduleForDruid::days)
}
