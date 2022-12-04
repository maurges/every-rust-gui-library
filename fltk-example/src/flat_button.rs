use std::{cell::Cell, rc::Rc};

use fltk::{widget::Widget, draw, enums::{FrameType, Color, Font, Align, Event}, prelude::{WidgetBase, WidgetExt}};

pub struct FlatButton {
    parent: Widget,
    pressed: Rc<Cell<bool>>,
}

fltk::widget_extends!(FlatButton, Widget, parent);

impl Default for FlatButton {
    fn default() -> Self {
        FlatButton::new(0, 0, 1, 1, "")
    }
}

impl FlatButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> FlatButton {
        let mut x = FlatButton {
            parent: Widget::new(x, y, w, h, None).with_label(label),
            pressed: Rc::new(Cell::new(false)),
        };
        x.draw();
        x.handle();
        x
    }

    // Overrides the draw function
    fn draw(&mut self) {
        let pressed = self.pressed.clone();
        self.parent.draw(move |b| {
            let pressed = pressed.get();
            let color = if pressed {
                Color::from_u32(0x7421fa)
            } else {
                Color::from_u32(0x304FFE)
            };
            draw::draw_box(
                FrameType::FlatBox,
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                color,
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
        let mut parent = self.parent.clone();
        let pressed = self.pressed.clone();
        self.parent.handle(move |_, ev| match ev {
            Event::Push => {
                pressed.set(true);
                parent.redraw();
                parent.do_callback();
                true
            }
            Event::Released => {
                pressed.set(false);
                parent.redraw();
                true
            }
            _ => false,
        });
    }
}
