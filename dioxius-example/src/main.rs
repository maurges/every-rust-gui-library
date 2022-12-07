#![allow(non_snake_case)]
mod todo_item;

use std::{cell::RefCell, rc::Rc};
use dioxus::prelude::{Scope, Element};

use todo_item::{TodoItem, TodoItemProps};

fn main() {
    dioxus::desktop::launch_with_props(app, 0, |x| x);
}

fn app(cx: Scope<i32>) -> Element {
    use dioxus::prelude::*;

    let shared_state = use_state(&cx, || Rc::new(RefCell::new(Vec::new())));
    let input_state = cx.use_hook(|_| Rc::new(RefCell::new(String::new())));
    let input_state_b = input_state.clone();

    cx.render(rsx! (
        div {
            input {
                r#type: "text",
                onchange: move |ev| { *input_state.borrow_mut() = ev.data.value.clone() }
            }
            button {
                onclick: move |_| {
                    shared_state.borrow_mut().push(TodoItemProps {
                        text: Rc::new(RefCell::new(input_state_b.borrow().clone())),
                        done: Rc::new(RefCell::new(false)),
                    });
                    cx.needs_update();
                },
                "add"
            }
        }

        shared_state.borrow().iter().map(|item| rsx!( TodoItem {
            text: item.text.clone(),
            done: item.done.clone(),
        } ))

        div {
            button {
                onclick: move |_| { save(&shared_state.borrow()); },
                "save"
            }
            button {
                onclick: move |_| { *shared_state.borrow_mut() = load(); cx.needs_update() },
                "load"
            }
        }
    ))
}

fn save(data: &[TodoItemProps]) {
    let data = data.iter().map(|x| TodoState {
        text: x.text.borrow().clone(),
        done: x.done.borrow().clone(),
    }).collect::<Vec<_>>();
    // haha
    let dest = "./test.ron";
    if let Ok(file) = std::fs::File::create(dest) {
        match ron::ser::to_writer(file, &data) {
            Ok(()) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn load() -> Vec<TodoItemProps> {
    let src = "./test.ron";
    let file = std::fs::File::open(src).unwrap();
    let data: Vec<TodoState> =
        ron::de::from_reader(file).unwrap();
    data.into_iter().map(|x| TodoItemProps {
        text: Rc::new(RefCell::new(x.text)),
        done: Rc::new(RefCell::new(x.done)),
    }).collect()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct TodoState {
    text: String,
    done: bool,
}
