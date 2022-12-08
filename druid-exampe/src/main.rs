use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("Hello World!")
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state: HelloState = HelloState {
        name: "World".into(),
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    let make = |text| Label::new(text).with_text_size(18.0);

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .must_fill_main_axis(true)
        .with_child(Flex::row()
            .with_child(make("topleft"))
            .with_child(make("topright"))
            .must_fill_main_axis(true)
        )
        .with_flex_spacer(0.0)
        .with_child(Flex::row()
            .with_child(make("botleftleft"))
            .with_child(make("botleftright"))
            .must_fill_main_axis(true)
        )
        .center()
}
