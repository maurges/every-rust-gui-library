use fltk::{widget::Widget, draw, enums::{FrameType, Color, Font, Align, Event}, prelude::{WidgetBase, WidgetExt}};

pub struct FlatButton {
    wid: Widget,
}

fltk::widget_extends!(FlatButton, Widget, wid);

impl Default for FlatButton {
    fn default() -> Self {
        FlatButton::new(0, 0, 1, 1, "")
    }
}

impl FlatButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> FlatButton {
        let mut x = FlatButton {
            wid: Widget::new(x, y, w, h, None).with_label(label),
        };
        x.draw();
        x.handle();
        x
    }

    // Overrides the draw function
    fn draw(&mut self) {
        self.wid.draw(move |b| {
            draw::draw_box(
                FrameType::FlatBox,
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                Color::from_u32(0x304FFE),
            );
            draw::set_draw_color(Color::White);
            draw::set_font(Font::Courier, 24);
            draw::draw_text2(
                &b.label(),
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                Align::Center,
            );
        });
    }

    // Overrides the handle function.
    // Notice the do_callback which allows the set_callback method to work
    fn handle(&mut self) {
        let mut wid = self.wid.clone();
        self.wid.handle(move |_, ev| match ev {
            Event::Push => {
                wid.do_callback();
                true
            }
            _ => false,
        });
    }
}
