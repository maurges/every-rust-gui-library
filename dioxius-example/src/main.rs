#![allow(non_snake_case)]
use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::{Scope, Element};

fn main() {
    dioxus::desktop::launch_with_props(app, 0, |x| x);
}

fn app(cx: Scope<i32>) -> Element {
    use dioxus::prelude::*;

    let shared_state = use_state(&cx, || Rc::new(RefCell::new(Vec::new())));
    let input_state = cx.use_hook(|_| Rc::new(RefCell::new(String::new())));
    let input_state_b = input_state.clone();

    cx.render(rsx! (
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
        shared_state.borrow().iter().map(|item| rsx!( TodoItem {
            text: item.text.clone(),
            done: item.done.clone(),
        } ))
        button {
            onclick: move |_| {
                for item in shared_state.borrow().iter() {
                    eprintln!("item: text = {}, done = {}", item.text.borrow(), item.done.borrow());
                }
            },
            "save"
        }
    ))
}

fn save(data: &[TodoItemProps]) {

}

/*
#[derive(Debug)]
struct TodoState {
    text: String,
    done: bool,
}
*/

#[derive(PartialEq, dioxus::prelude::Props)]
struct TodoItemProps {
    text: Rc<RefCell<String>>,
    done: Rc<RefCell<bool>>,
}

fn TodoItem(cx: Scope<TodoItemProps>) -> Element {
    use dioxus::prelude::*;

    let times = cx.use_hook(|_| 0_i64);
    let done: &bool = &cx.props.done.borrow();
    let text: &String = &cx.props.text.borrow();

    cx.render(rsx!(
        div {
            input {
                r#type: "checkbox",
                checked: "{done}",
                onchange: move |ev| { *cx.props.done.borrow_mut() = ev.data.value == "true" }
            }
            "{text}"
            button {
                onclick: move |_| {
                    *times += 1;
                    println!("clicked {} times", times);
                },
                "edit"
            }
        }
    ))
}
