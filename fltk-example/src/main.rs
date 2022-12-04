mod flat_button;
mod todo_item;

use fltk::prelude::{WidgetExt, WindowExt, GroupExt, InputExt};
use fltk::{app, window::Window};


#[derive(Clone)]
pub enum ButtonMessage {
    Add,
    Save,
    Load,
}

fn main() {
    let app = app::App::default()
        .with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(500, 800)
        .center_screen()
        .with_label("Counter");

    let mut scroll = fltk::group::Scroll::default()
        .size_of(&wind);
    scroll.set_color(wind.color());

    let new_input = fltk::input::Input::default()
        .with_pos(0, 0)
        .with_size(400, 30)
        .with_label("new task here");
    let mut new_button = fltk::button::Button::default()
        .with_size(40, 30)
        .right_of(&new_input, 5)
        .with_label("add");

    let mut save_button = fltk::button::Button::default()
        .with_size(40, 30)
        .below_of(&new_input, 5)
        .with_label("save");
    let mut load_button = fltk::button::Button::default()
        .with_size(40, 30)
        .right_of(&save_button, 5)
        .with_label("load");


    let mut col_children = Vec::new();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let (s, r) = app::channel::<ButtonMessage>();
    new_button.emit(s.clone(), ButtonMessage::Add);
    save_button.emit(s.clone(), ButtonMessage::Save);
    load_button.emit(s.clone(), ButtonMessage::Load);

    let mut dummy_widget = todo_item::TodoItem::new(
        0, new_input.height(),
        450, 30,
        "test".to_owned(), false,
    );
    scroll.end();
    dummy_widget.show();


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
                        new_input.value(),
                        false,
                    );
                    scroll.add(&*new_child);
                    wind.redraw();
                    col_children.push(new_child);
                    prev_child = col_children.last().unwrap();
                }
                ButtonMessage::Save => {
                    let mut dialog = fltk::dialog::FileDialog::new(fltk::dialog::FileDialogType::BrowseSaveFile);
                    dialog.show();
                    let dest = dialog.filename();

                    let data = col_children.iter().map(|c| c.get_state()).collect::<Vec<_>>();
                    if let Ok(file) = std::fs::File::create(dest) {
                        match ron::ser::to_writer(file, &data) {
                            Ok(()) => (),
                            Err(e) => fltk::dialog::message_default(&format!("{}", e)),
                        }
                    }
                }
                ButtonMessage::Load => {
                    let mut dialog = fltk::dialog::FileDialog::new(fltk::dialog::FileDialogType::BrowseFile);
                    dialog.show();
                    let src = dialog.filename();
                    let file = std::fs::File::open(src).unwrap();
                    let data: Vec<todo_item::TodoItemState> =
                        ron::de::from_reader(file).unwrap();
                    // remove existing items
                    prev_child = &dummy_widget;
                    for child in col_children.drain(0..) {
                        child.delete();
                    }
                    // add new items
                    for item in data {
                        let new_child = todo_item::TodoItem::new(
                            prev_child.x(),
                            prev_child.y() + prev_child.height() + 5,
                            450,
                            30,
                            item.message,
                            item.done,
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
}
