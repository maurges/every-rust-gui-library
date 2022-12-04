use std::cell::RefCell;
use std::rc::Rc;

use fltk::widget::Widget;
use fltk::prelude::{WidgetExt, WidgetBase};

pub struct TodoItemState {
    message: String,
    done: bool,
}

pub struct TodoItem {
    parent: Widget,
    checkbox: fltk::button::CheckButton,
    label: fltk::frame::Frame,
    edit_button: fltk::button::Button,
    state: Rc<RefCell<TodoItemState>>,
}

fltk::widget_extends!(TodoItem, Widget, parent);

impl Default for TodoItem {
    fn default() -> Self {
        TodoItem::new(0, 0, 0, 0, "".to_owned(), false)
    }
}

impl TodoItem {
    pub fn new(x: i32, y: i32, w: i32, h: i32, message: String, done: bool) -> Self {
        let state = TodoItemState { message, done };
        let mut r = TodoItem {
            parent: Widget::new(x, y, w, h, None),
            checkbox: fltk::button::CheckButton::new(x, y, w, h, None),
            label: fltk::frame::Frame::new(x, y, w, h, None)
                .with_label(&state.message),
            edit_button: fltk::button::Button::new(x, y, w, h, None)
                .with_label("edit"),
            state: Rc::new(RefCell::new(state)),
        };
        let mut cb = r.checkbox.clone();
        let mut l = r.label.clone();
        let mut eb = r.edit_button.clone();
        r.parent.draw(move |_| {
            cb.redraw();
            l.redraw();
            eb.redraw();
        });
        r.resize(x, y, w, h);
        r
    }

    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if w == 0 || h == 0 { return; }
        self.parent.resize(x, y, w, h);
        self.checkbox.resize(
            x, y,
            core::cmp::min(w, h),
            core::cmp::min(w, h),
        );
        let ew = core::cmp::min(w, h * 2);
        self.edit_button.resize(
            x + w - ew,
            y,
            ew,
            h,
        );
        take_mut::take(&mut self.label, |l| l
            .with_size(core::cmp::max(0, w - ew - h), h)
            .right_of(&self.checkbox, 0)
        );
        eprintln!("checkbox pos: {} x {} with {} x {}", self.checkbox.x(), self.checkbox.y(), self.checkbox.width(), self.checkbox.height());
        eprintln!("label pos: {} x {} with {} x {}", self.label.x(), self.label.y(), self.label.width(), self.label.height());
        eprintln!("edit pos: {} x {} with {} x {}", self.edit_button.x(), self.edit_button.y(), self.edit_button.width(), self.edit_button.height());
    }
    pub fn widget_resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.resize(x, y, w, h)
    }

    pub fn with_message(mut self, message: String) -> Self {
        {
        let mut state = self.state.borrow_mut();
        state.message = message;
        self.label.set_label(&state.message);
        }
        self
    }
}
