use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window, enums::Color};
use std::{rc::Rc, cell::Cell};

fn main() {
    let state = Rc::new(Cell::new(0_i64));
    let state_rendered = Rc::new(Cell::new("0".to_owned()));

    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(160, 200)
        .center_screen()
        .with_label("Counter");
    wind.set_color(Color::White);
    let mut frame = Frame::default()
        .with_size(100, 40)
        .center_of(&wind)
        .with_label("0");
    let mut but_inc = Button::default()
        .size_of(&frame)
        .above_of(&frame, 0)
        .with_label("+");
    but_inc.set_color(Color::White.darker());
    let mut but_dec = Button::default()
        .size_of(&frame)
        .below_of(&frame, 0)
        .with_label("-");
    but_dec.set_color(Color::White.darker());
    wind.make_resizable(true);
    wind.end();
    wind.show();

    /* Event handling */
    let mut set_state = |x| {
        state.set(x);
        state_rendered.set(format!("{}", x));
        frame.set_label(todo!());
    };

    app.run().unwrap();
}
