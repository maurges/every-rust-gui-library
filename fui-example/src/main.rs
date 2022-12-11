#![windows_subsystem = "windows"]

use anyhow::{Error, Result};
use fui_app::{Application, Window, WindowOptions};
use fui_controls::{Button, Text, TextBox, ToggleButton};
use fui_core::{
    Callback, Children, ControlObject, Horizontal, Property, Style,
    Vertical, ViewContext, ViewModel, ObservableVec,
};
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::Rc;
use tokio::task::LocalSet;

use typed_builder::TypedBuilder;
use typemap::TypeMap;

struct MainViewModel {
    add_new: Property<String>,
    items: ObservableVec<TodoItem>,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MainViewModel {
            add_new: Property::new("Enter note here".to_owned()),
            items: ObservableVec::new(),
        }))
    }
}

#[derive(TypedBuilder)]
pub struct ButtonText {
    #[builder(default = Property::new("".to_string()))]
    pub text: Property<String>,
    #[builder(default = Callback::empty())]
    pub clicked: Callback<()>,
}

#[derive(TypedBuilder, Clone)]
struct TodoItem {
    #[builder(default = Property::new("".to_string()))]
    pub text: Property<String>,
    #[builder(default = Property::new(false))]
    pub done: Property<bool>,
    #[builder(default = Property::new(false))]
    editing: Property<bool>,
}

impl ViewModel for TodoItem {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm: &mut Self = &mut view_model.borrow_mut();
        ui! (
            Horizontal {
                ToggleButton {
                    is_checked: &vm.done,
                },
                Text {
                    text: &vm.text,
                },
                Button {
                    Text {
                        text: (&vm.editing, |is| if is { "done".to_owned() } else { "edit".to_owned() }),
                    },
                    clicked: Callback::new_vm(view_model, |vm, _| vm.editing.change(|x| !x)),
                },
            }
        )
    }
}

impl ButtonText {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        ui! {
            Button {
                clicked: self.clicked,
                Text { text: self.text }
            }
        }
    }
}

impl ViewModel for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();

        let todo_item = TodoItem {
            text: Property::new("foobar".to_owned()),
            done: Property::new(false),
            editing: Property::new(false),
        };
        let todo_item = Rc::new(RefCell::new(todo_item));
        let todo_view: Property<Rc<RefCell<dyn ControlObject>>> =
            Property::new(TodoItem::create_view(&todo_item));

        ui!(
            Vertical {
                Horizontal {
                    TextBox {
                        text: &vm.add_new,
                    },
                    Button {
                        Text { text: "Add" }
                    }
                },
                &vm.items
            }
        )
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    LocalSet::new()
        .run_until(async {
            let app = Application::new("Example: button").await?;

            let mut window = Window::create(
                WindowOptions::new()
                    .with_title("Example: button")
                    .with_size(800, 600),
            )
            .await?;

            window.set_vm(MainViewModel::new());

            app.run().await?;

            Ok::<(), Error>(())
        })
        .await
}
