// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Counter example (simple button)

mod todo_item;

use kas::prelude::{impl_scope, EventMgr, Widget, HasStr};
use kas::widgets;

use todo_item::TodoItem;

impl_scope! {
    #[widget{
        layout = column: [
            row: [
                self.text_field_add,
                self.button_add,
            ],
            self.main_list,
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
        main_list: widgets::List<kas::dir::Down,TodoItem> ,
    }
    impl Widget for Self {
        fn handle_message(&mut self, mgr: &mut EventMgr, _: usize) {
            if let Some(ItemAdd) = mgr.try_pop_msg() {
                mgr.config_mgr(|mgr|
                    self.main_list.push(mgr, TodoItem::new(self.text_field_add.get_string()))
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ItemAdd;

impl MainWindow {
    fn new() -> Self {
        let text_field_add = widgets::EditBox::new("");
        let button_add = widgets::TextButton::new_msg("Add", ItemAdd);
        let main_list = widgets::List::new();

        MainWindow {
            core: Default::default(),

            text_field_add,
            button_add,
            main_list,
        }
    }
}
impl kas::Window for MainWindow {
    fn title(&self) -> &str {
        "Todos"
    }
}

fn main() -> kas::shell::Result<()> {
    let theme = kas::theme::SimpleTheme::new().with_font_size(24.0);

    kas::shell::Toolkit::new(theme)?
        .with(MainWindow::new())?
        .run()
}
