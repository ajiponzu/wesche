use super::super::core::day::Day;
use super::super::core::schedule::Schedule;
use super::super::core::task::Task;

use druid::text::FontDescriptor;
use druid::widget::{Align, Button, Flex, Label, List, ViewSwitcher};
use druid::{
    im, lens, AppLauncher, Color, Data, FontFamily, FontStyle, Lens, LensExt, Widget, WidgetExt,
    WindowDesc,
};

#[derive(Data, Clone, Lens)]
struct TaskForDruid {
    pub id: usize,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
    pub details: String,
    pub is_clicked: bool,
}

#[derive(Data, Clone, Lens)]
struct DayForDruid {
    pub id: usize,
    pub day_of_week: String,
    pub tasks: im::Vector<TaskForDruid>,
    pub is_clicked: bool,
}

#[derive(Data, Clone, Lens)]
struct ScheduleForDruid {
    pub days: im::Vector<DayForDruid>,
}

#[derive(Data, Clone, Lens)]
struct App {
    pub view_mode: usize,
    pub selected_day: usize,
    pub selected_task: usize,
    schedule: ScheduleForDruid,
}

impl TaskForDruid {
    fn from(task: &Task, id: usize) -> TaskForDruid {
        TaskForDruid {
            id,
            title: task.get_title().to_string(),
            start_time: task.get_start_time().to_string(),
            end_time: task.get_end_time().to_string(),
            details: task.get_details().to_string(),
            is_clicked: false,
        }
    }
}

impl DayForDruid {
    fn from(day: &Day, id: usize) -> DayForDruid {
        DayForDruid {
            id,
            day_of_week: day.get_day_of_week().to_string(),
            tasks: day
                .get_tasks()
                .iter()
                .enumerate()
                .map(|(idx, task)| TaskForDruid::from(task, idx))
                .collect(),
            is_clicked: false,
        }
    }
}

impl ScheduleForDruid {
    fn from(schedule: &Schedule) -> ScheduleForDruid {
        ScheduleForDruid {
            days: schedule
                .get_days()
                .iter()
                .enumerate()
                .map(|(idx, day)| DayForDruid::from(day, idx))
                .collect(),
        }
    }
}

impl App {
    fn from(schedule: ScheduleForDruid) -> App {
        App {
            view_mode: 0,
            selected_day: 0,
            selected_task: 0,
            schedule,
        }
    }
}

pub fn open_window(window_title: &str, schedule: Schedule) {
    let main_window = WindowDesc::new(build_ui())
        .title(window_title)
        .window_size((820.0, 600.0))
        .resizable(false)
        .transparent(true);

    let app = App::from(ScheduleForDruid::from(&schedule));

    AppLauncher::with_window(main_window)
        .launch(app)
        .expect("Failed to launch application");
}

fn build_schedule_ui() -> impl Widget<App> {
    List::new(|| {
        Flex::row()
            .with_child(
                Label::new(|day: &DayForDruid, _env: &_| day.day_of_week.to_string())
                    .with_font(FontDescriptor::new(FontFamily::SERIF).with_style(FontStyle::Italic))
                    .with_text_color(Color::BLACK)
                    .with_text_size(18.0),
            )
            .center()
            .fix_size(100.0, 100.0)
            .background(Color::rgba8(255, 255, 255, 255))
            .on_click(|_event, day: &mut DayForDruid, _env| {
                day.is_clicked = true;
            })
            .border(Color::BLUE, 2.0)
            .padding(5.0)
    })
    .horizontal()
    .lens(lens::Identity.map(
        |app: &App| app.schedule.days.clone(),
        |app: &mut App, days| {
            if let Some(selected_day) = days.iter().position(|day| day.is_clicked) {
                app.view_mode = 1;
                app.selected_day = selected_day;
            }
        },
    ))
    .center()
    .background(Color::rgba8(0, 0, 0, 180))
}

fn build_today_ui(selected_day_index: usize) -> impl Widget<App> {
    Flex::row()
        .with_child(
            Flex::column()
                .with_child(
                    Button::new("◀")
                        .on_click(|_event, app: &mut App, _env| {
                            app.view_mode = 0;
                        })
                        .fix_width(60.0)
                        .fix_height(600.0)
                        .center(),
                )
                .padding(5.0),
        )
        .with_child(
            Flex::row()
                .with_child(Flex::column().fix_width(100.0).expand_height())
                .with_child(
                    List::new(|| {
                        Flex::column()
                            .with_child(
                                Label::new(|task: &TaskForDruid, _env: &_| {
                                    format!("★ {}", task.title)
                                })
                                .with_font(
                                    FontDescriptor::new(FontFamily::SERIF)
                                        .with_style(FontStyle::Italic),
                                )
                                .with_text_color(Color::RED)
                                .with_text_size(20.0)
                                .padding(10.0)
                                .expand_width()
                                .background(Color::rgba8(230, 200, 250, 200))
                                .padding(5.0),
                            )
                            .with_child(
                                Label::new(|task: &TaskForDruid, _env: &_| {
                                    format!("⌛ {} ～ {}", task.start_time, task.end_time)
                                })
                                .with_text_color(Color::BLACK)
                                .with_text_size(20.0)
                                .padding(10.0)
                                .expand_width()
                                .background(Color::rgba8(200, 255, 240, 200))
                                .padding(5.0),
                            )
                            .center()
                            .fix_size(560.0, 120.0)
                            .background(Color::rgba8(255, 255, 255, 230))
                            .on_click(|_event, task: &mut TaskForDruid, _env| {
                                task.is_clicked = true;
                            })
                            .border(Color::BLUE, 2.0)
                            .padding(10.0)
                    })
                    .lens(lens::Identity.map(
                        move |app: &App| app.schedule.days[selected_day_index].tasks.clone(),
                        |app: &mut App, tasks| {
                            if let Some(selected_task) =
                                tasks.iter().position(|task| task.is_clicked)
                            {
                                app.view_mode = 2;
                                app.selected_task = selected_task;
                            }
                        },
                    ))
                    .fix_size(560.0, 600.0),
                )
                .with_child(Flex::column().fix_width(80.0).expand_height())
                .scroll()
                .vertical(),
        )
        .background(Color::rgba8(250, 240, 220, 180))
}

fn build_task_ui(selected_day_index: usize, selected_task_index: usize) -> impl Widget<App> {
    Flex::row()
        .with_child(
            Flex::column()
                .with_child(
                    Button::new("◀")
                        .on_click(|_event, app: &mut App, _env| {
                            app.view_mode = 1;
                        })
                        .fix_width(60.0)
                        .fix_height(600.0)
                        .center(),
                )
                .padding(5.0),
        )
        .with_child(Flex::column().fix_width(100.0).expand_height())
        .with_child(
            Flex::column()
                .with_child(
                    Label::new(|task: &TaskForDruid, _env: &_| format!("★ {}", task.title))
                        .with_font(
                            FontDescriptor::new(FontFamily::SERIF).with_style(FontStyle::Italic),
                        )
                        .with_text_color(Color::RED)
                        .with_text_size(18.0)
                        .padding(10.0)
                        .expand_width()
                        .background(Color::rgba8(230, 200, 250, 200))
                        .padding(5.0),
                )
                .with_child(
                    Label::new(|task: &TaskForDruid, _env: &_| {
                        format!("⌛ {} ～ {}", task.start_time, task.end_time)
                    })
                    .with_text_color(Color::BLACK)
                    .with_text_size(18.0)
                    .padding(10.0)
                    .expand_width()
                    .background(Color::rgba8(200, 255, 240, 200))
                    .padding(5.0),
                )
                .with_child(
                    Flex::row()
                        .with_child(
                            Label::new(|task: &TaskForDruid, _env: &_| {
                                format!("【📖詳細】\n👇\n{}", task.details)
                            })
                            .with_font(FontDescriptor::new(FontFamily::SERIF).with_size(16.0))
                            .with_text_color(Color::BLACK)
                            .fix_size(490.0, 500.0)
                            .background(Color::rgba8(230, 245, 255, 180)),
                        )
                        .scroll()
                        .fix_height(430.0),
                )
                .fix_size(500.0, 600.0)
                .background(Color::rgba8(255, 255, 255, 230))
                .padding(10.0)
                .lens(
                    App::schedule
                        .then(ScheduleForDruid::days)
                        .index(selected_day_index)
                        .then(DayForDruid::tasks)
                        .index(selected_task_index),
                ),
        )
        .with_child(Flex::column().fix_width(80.0).expand_height())
        .expand()
        .background(Color::rgba8(250, 240, 220, 180))
}

fn build_ui() -> impl Widget<App> {
    let view_switcher = ViewSwitcher::new(
        |app: &App, _env| app.view_mode,
        |selector, app, _env| match selector {
            1 => Box::new(build_today_ui(app.selected_day)),
            2 => Box::new(build_task_ui(app.selected_day, app.selected_task)),
            _ => Box::new(build_schedule_ui()),
        },
    );

    Align::centered(view_switcher)
}
