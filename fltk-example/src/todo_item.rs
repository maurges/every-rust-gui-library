use std::cell::RefCell;
use std::rc::Rc;

use fltk::widget::Widget;
use fltk::prelude::{WidgetExt, WidgetBase, InputExt};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TodoItemState {
    pub message: String,
    pub done: bool,
}

pub struct TodoItem {
    parent: Widget,
    checkbox: fltk::button::CheckButton,
    label: fltk::frame::Frame,
    edit_label: fltk::input::Input,
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
        let state = Rc::new(RefCell::new(TodoItemState {
            message,
            done,
        }));
        let is_editing = Rc::new(RefCell::new(false));
        let mut r = TodoItem {
            parent: Widget::new(x, y, w, h, None),
            checkbox: fltk::button::CheckButton::new(x, y, w, h, None),
            label: fltk::frame::Frame::new(x, y, w, h, None)
                .with_label(&state.borrow().message),
            edit_label: fltk::input::Input::new(x, y, w, h, None),
            edit_button: fltk::button::Button::new(x, y, w, h, None)
                .with_label("edit"),
            state: state.clone(),
        };
        r.edit_label.set_value(&state.borrow().message);
        r.edit_label.hide();

        // setup redraw
        let mut cb = r.checkbox.clone();
        let mut l = r.label.clone();
        let mut eb = r.edit_button.clone();
        r.parent.draw(move |_| {
            cb.redraw();
            l.redraw();
            eb.redraw();
        });
        r.resize(x, y, w, h);

        // setup editing
        let mut l = r.label.clone();
        let mut el = r.edit_label.clone();
        let mut eb = r.edit_button.clone();
        let s = state.clone();
        r.edit_button.set_callback(move |_| {
            let mut state = s.borrow_mut();
            let mut is_editing = is_editing.borrow_mut();
            *is_editing = !*is_editing;
            if *is_editing {
                eb.set_label("done");
                l.hide();
                el.show();
            } else {
                eb.set_label("edit");
                l.show();
                el.hide();
                state.message = el.value();
                l.set_label(&state.message);
            }
        });

        let cb = r.checkbox.clone();
        r.checkbox.set_callback(move |_| {
            let mut state = state.borrow_mut();
            state.done = cb.is_checked();
        });

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
        self.edit_label.resize(
            self.label.x(),
            self.label.y(),
            self.label.width(),
            self.label.height(),
        );
        eprintln!("checkbox pos: {} x {} with {} x {}", self.checkbox.x(), self.checkbox.y(), self.checkbox.width(), self.checkbox.height());
        eprintln!("label pos: {} x {} with {} x {}", self.label.x(), self.label.y(), self.label.width(), self.label.height());
        eprintln!("edit pos: {} x {} with {} x {}", self.edit_button.x(), self.edit_button.y(), self.edit_button.width(), self.edit_button.height());
    }

    pub fn get_state(&self) -> TodoItemState {
        self.state.borrow().clone()
    }

    pub fn delete(self) {
        <Widget as WidgetBase>::delete(self.parent);
        <fltk::button::CheckButton as WidgetBase>::delete(self.checkbox);
        <fltk::frame::Frame as WidgetBase>::delete(self.label);
        <fltk::input::Input as WidgetBase>::delete(self.edit_label);
        <fltk::button::Button as WidgetBase>::delete(self.edit_button);
    }
}
