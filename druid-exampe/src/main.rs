mod todo_item;

use druid::im;
use druid::widget;
use druid::widget::prelude::*;
use druid::{AppLauncher, Data, Lens, WidgetExt, WindowDesc};

use todo_item::{todo_item, TodoItem, TodoItemState};

#[derive(Clone, Data, Lens)]
struct AppState {
    add_new: String,
    todo_items: im::Vector<TodoItemState>,
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("Hello World!")
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = AppState {
        add_new: String::new(),
        todo_items: vec![
            TodoItemState::new(TodoItem {
                text: "foo".into(),
                done: false,
            }),
            TodoItemState::new(TodoItem {
                text: "bar".into(),
                done: true,
            }),
        ]
        .into(),
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    let add_button = widget::Button::new("Add").on_click(|_ctx, state: &mut AppState, _env| {
        state.todo_items.push_back(TodoItemState::new(TodoItem {
            text: state.add_new.clone(),
            done: false,
        }));
    });

    let save_button = widget::Button::new("Save").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(
            druid::FileDialogOptions::new()
        ))
    });

    let load_button = widget::Button::new("Load").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(
            druid::FileDialogOptions::new()
        ))
    });

    widget::Scroll::new(
        widget::Flex::column()
            .with_child(
                widget::Flex::row()
                    .with_child(
                        widget::TextBox::new()
                            .with_placeholder("Add note")
                            .lens(AppState::add_new),
                    )
                    .with_child(add_button)
                    .center(),
            )
            .with_child(widget::List::new(|| todo_item()).lens(AppState::todo_items))
            .with_child(widget::Flex::row()
                .with_child(save_button)
                .with_child(load_button)
            )
            .center(),
    )
}
