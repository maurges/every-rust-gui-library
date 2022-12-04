mod flat_button;
mod column_layout;

use fltk::prelude::{WidgetExt, WindowExt, GroupExt};
use fltk::{app, frame::Frame, window::Window, enums::Color};


#[derive(Clone)]
pub enum ButtonMessage {
    Inc,
    Dec,
    Add,
}

fn main() {
    let mut state = 0_i64;
    let mut state_rendered;

    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(160, 200)
        .center_screen()
        .with_label("Counter");
    wind.set_color(Color::White);

    let mut scroll = fltk::group::Scroll::default()
        .size_of(&wind);
    scroll.set_color(Color::White);
    let mut col = column_layout::ColumnLayout::default();
    scroll.end();

    let mut frame = Frame::default()
        .with_size(100, 40)
        .with_label("0");
    let mut but_inc = flat_button::FlatButton::default()
        .size_of(&frame)
        .with_label("+");
    let mut but_dec = flat_button::FlatButton::default()
        .size_of(&frame)
        .with_label("-");

    let mut but_add = flat_button::FlatButton::default()
        .size_of(&frame)
        .with_label("add");

    col.add(&mut but_inc);
    col.add(&mut &mut frame); // lol
    col.add(&mut but_dec);
    col.add(&mut but_add);

    let mut col_children = Vec::new();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    eprintln!("scroll children: {}", scroll.children());
    eprintln!("window children: {}", wind.children());

    let (s, r) = app::channel::<ButtonMessage>();

    but_inc.emit(s.clone(), ButtonMessage::Inc);
    but_dec.emit(s.clone(), ButtonMessage::Dec);
    but_add.emit(s, ButtonMessage::Add);

    /* Event handling */
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                ButtonMessage::Inc => { state += 1; },
                ButtonMessage::Dec => { state -= 1; }
                ButtonMessage::Add => {
                    let mut new_child = Frame::default()
                        .size_of(&frame)
                        .with_label("chlen");
                    col.add(&mut &mut new_child);
                    wind.redraw();
                    eprintln!("child is at: {} x {}", new_child.x(), new_child.y());
                    col_children.push(new_child);
                }
            }
            state_rendered = format!("{}", state);
            frame.set_label(&state_rendered);
        }
    }
}
