mod flat_button;

use std::ops::Deref;

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

    let mut frame = Frame::default()
        .with_size(100, 40)
        .center_of(&wind)
        .with_label("0");
    let mut but_inc = flat_button::FlatButton::default()
        .size_of(&frame)
        .above_of(&frame, 0)
        .with_label("+");
    let mut but_dec = flat_button::FlatButton::default()
        .size_of(&frame)
        .below_of(&frame, 0)
        .with_label("-");
    let mut but_add = flat_button::FlatButton::default()
        .size_of(&frame)
        .below_of(but_dec.deref(), 0)
        .with_label("add");
    scroll.end();

    let mut col_children = Vec::new();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let (s, r) = app::channel::<ButtonMessage>();

    but_inc.emit(s.clone(), ButtonMessage::Inc);
    but_dec.emit(s.clone(), ButtonMessage::Dec);
    but_add.emit(s, ButtonMessage::Add);

    let dummy_frame = Frame::default()
        .with_size(1, 1)
        .below_of(&*but_add, 0);
    /* Event handling */
    let mut prev_child = &dummy_frame;
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                ButtonMessage::Inc => { state += 1; },
                ButtonMessage::Dec => { state -= 1; }
                ButtonMessage::Add => {
                    let new_child = Frame::default()
                        .size_of(&frame)
                        .below_of(prev_child, 0)
                        .with_label("chlen");
                    scroll.add(&new_child);
                    wind.redraw();
                    col_children.push(new_child);
                    prev_child = col_children.last().unwrap();
                }
            }
            state_rendered = format!("{}", state);
            frame.set_label(&state_rendered);
        }
    }
}
