// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Counter example (simple button)

mod todo_item;

use kas::prelude::{impl_scope, EventMgr, Widget, HasStr};
use kas::widgets;

use todo_item::{TodoItem, TodoState};

impl_scope! {
    #[widget{
        layout = column: [
            row: [
                self.text_field_add,
                self.button_add,
            ],
            self.main_list,
            row: [
                self.button_save,
                self.button_load,
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
        main_list: widgets::List<kas::dir::Down,TodoItem> ,
        #[widget]
        button_save: widgets::TextButton,
        #[widget]
        button_load: widgets::TextButton,
    }
    impl Widget for Self {
        fn handle_message(&mut self, mgr: &mut EventMgr, _: usize) {
            if let Some(ItemAdd) = mgr.try_pop_msg() {
                mgr.config_mgr(|mgr|
                    self.main_list.push(mgr, TodoItem::new(self.text_field_add.get_string()))
                );
            }
            if let Some(Save) = mgr.try_pop_msg() {
                let items = self.main_list.iter().map(|x| x.get_state()).collect::<Vec<_>>();
                save(&items);
            }
            if let Some(Load) = mgr.try_pop_msg() {
                if let Some(items) = load() {
                    self.main_list.clear();
                    mgr.config_mgr(|cgr| for item in &items {
                        let mut w = TodoItem::new(item.text.clone());
                        *cgr |= w.set_done(item.done);
                        self.main_list.push(cgr, w);
                    })
                }
            }
        }
    }
}

fn save(items: &[TodoState]) {
    if let Some(path) = rfd::FileDialog::new().save_file() {
        match std::fs::File::create(&path) {
            Ok(file) => match ron::ser::to_writer(file, items) {
                Ok(()) => (),
                Err(e) => eprintln!("{}", e),
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn load() -> Option<Vec<TodoState>> {
    let path = rfd::FileDialog::new().pick_file()?;
    match std::fs::File::open(path) {
        Err(e) => { eprintln!("{}", e); None }
        Ok(file) => match ron::de::from_reader(file) {
            Ok(items) => Some(items),
            Err(e) => { eprintln!("{}", e); None }
        }
    }
}

#[derive(Debug, Clone)]
struct ItemAdd;
#[derive(Debug, Clone)]
struct Save;
#[derive(Debug, Clone)]
struct Load;

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

            button_save: widgets::TextButton::new_msg("Save", Save),
            button_load: widgets::TextButton::new_msg("Load", Load),
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
