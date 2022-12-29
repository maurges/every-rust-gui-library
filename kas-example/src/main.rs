// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Counter example (simple button)

mod shared_cell;
mod todo_item;

use kas::prelude::{impl_scope, EventMgr, Widget};
use kas::view::{Driver, MaybeOwned};
use kas::widgets;

use shared_cell::SharedCell;
use todo_item::{TodoItem, TodoState};

#[derive(Debug)]
struct TodoDriver;

type RefVec<T> = SharedCell<Vec<T>>;

impl Driver<TodoState, RefVec<TodoState>> for TodoDriver {
    type Widget = TodoItem;

    fn make(&self) -> Self::Widget {
        TodoItem::new()
    }

    fn set_mo(
        &self,
        widget: &mut Self::Widget,
        _key: &usize,
        item: MaybeOwned<TodoState>,
    ) -> kas::TkAction {
        let data = item.into_owned();
        widget.set_done(data.done) | widget.set_text(data.text)
    }

    fn on_message(
        &self,
        mgr: &mut EventMgr,
        _widget: &mut TodoItem,
        data: &RefVec<TodoState>,
        key: &usize,
    ) {
        if let Some(todo_item::CheckToggle(val)) = mgr.try_pop_msg() {
            let mut data = data.cell.borrow_mut();
            data[*key].done = val;
        }
    }
}

impl Default for TodoDriver {
    fn default() -> Self {
        TodoDriver
    }
}

type ListView<T, D> = kas::view::ListView<kas::dir::Direction, T, D>;

impl_scope! {
    #[widget{
        layout = column: [
            row: [
                self.text_field_add,
                self.button_add,
            ]
        ];
    }]
    #[derive(Debug)]
    struct MainWindow {
        core: widget_core!(),

        #[widget]
        text_field_add: widgets::EditBox,
        #[widget]
        button_add: widgets::TextButton,
        #[widget]
        main_list: ListView< RefVec<TodoState>, TodoDriver >,
    }
    impl Widget for Self {
        fn handle_message(&mut self, _mgr: &mut EventMgr, _: usize) {

        }
    }
}

#[derive(Debug, Clone)]
struct ItemAdd;

impl MainWindow {
    fn new() -> Self {
        let data = SharedCell::new(Vec::new());

        let text_field_add = widgets::EditBox::new("");
        let button_add = widgets::TextButton::new_msg("Add", ItemAdd);
        let main_list = ListView::new_with_direction(kas::dir::Direction::Down, data);

        MainWindow {
            core: Default::default(),

            text_field_add,
            button_add,
            main_list,
        }
    }
}

impl MainWindow {

}

fn main() -> kas::shell::Result<()> {
    let theme = kas::theme::SimpleTheme::new().with_font_size(24.0);

    let window = MainWindow::new();

    kas::shell::Toolkit::new(theme)?
        .with(kas::widgets::dialog::Window::new("Todos", window))?
        .run()
}
