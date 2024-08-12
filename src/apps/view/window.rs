use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
pub struct ScheduleWindowState {
    name: String,
}

fn build_root_widget() -> impl Widget<ScheduleWindowState> {
    use druid::widget::{Button, Flex, Label};

    let label = Label::new(|data: &ScheduleWindowState, _env: &_| data.name.clone());
    let button = Button::new("Click me!");

    Flex::column().with_child(label).with_child(button)
}

pub fn open_window(window_title: &str, window_size: (f64, f64)) {
    let main_window = WindowDesc::new(build_root_widget())
        .title(window_title)
        .window_size(window_size);

    let app = AppLauncher::with_window(main_window);

    app.launch(ScheduleWindowState {
        name: "Hello, world!".to_string(),
    })
    .expect("launch failed");
}
