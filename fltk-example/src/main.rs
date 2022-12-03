use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window, enums::Color};

#[derive(Clone)]
pub enum ButtonMessage {
    Inc,
    Dec,
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

    let (s, r) = app::channel::<ButtonMessage>();

    but_inc.emit(s.clone(), ButtonMessage::Inc);
    but_dec.emit(s, ButtonMessage::Dec);

    /* Event handling */
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                ButtonMessage::Inc => { state += 1; },
                ButtonMessage::Dec => { state -= 1; }
            }
            state_rendered = format!("{}", state);
            frame.set_label(&state_rendered);
        }
    }
}
