mod flat_button;
mod todo_item;

use fltk::prelude::{WidgetExt, WindowExt, GroupExt};
use fltk::{app, window::Window, enums::Color};


#[derive(Clone)]
pub enum ButtonMessage {
    Add,
}

fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(500, 800)
        .center_screen()
        .with_label("Counter");
    wind.set_color(Color::White);

    let mut scroll = fltk::group::Scroll::default()
        .size_of(&wind);

    let new_input = fltk::input::Input::default()
        .with_pos(0, 0)
        .with_size(400, 30)
        .with_label("new task here");
    let mut new_button = fltk::button::Button::default()
        .with_size(40, 30)
        .right_of(&new_input, 5)
        .with_label("add");

    scroll.end();

    let mut col_children = Vec::new();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let (s, r) = app::channel::<ButtonMessage>();
    new_button.emit(s, ButtonMessage::Add);

    let dummy_widget = todo_item::TodoItem::new(
        0, new_input.height(),
        450, 30,
        "test".to_owned(), false,
    );
    /* Event handling */
    let mut prev_child = &dummy_widget;
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                ButtonMessage::Add => {
                    let new_child = todo_item::TodoItem::new(
                        prev_child.x(),
                        prev_child.y() + prev_child.height() + 5,
                        450,
                        30,
                        new_input.label().to_owned(),
                        false,
                    );
                    scroll.add(&*new_child);
                    wind.redraw();
                    col_children.push(new_child);
                    prev_child = col_children.last().unwrap();
                }
            }
        }
    }
}
