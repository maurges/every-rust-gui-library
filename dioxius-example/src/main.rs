#![allow(non_snake_case)]
use std::{cell::RefCell, rc::Rc, ops::Deref};

use dioxus::prelude::{Scope, Element, EventHandler};

fn main() {
    dioxus::desktop::launch_with_props(app, 0, |x| x);
}

fn app(cx: Scope<i32>) -> Element {
    use dioxus::prelude::*;

    let make_shared_state = || Rc::new(RefCell::new(vec![
        TodoState { text: "example".into(), done: true },
        TodoState { text: "example2".into(), done: false },
    ]));
    let shared_state = use_state(&cx, make_shared_state);
    cx.use_hook(|_| cx.provide_context(SharedTodoState(shared_state.deref().clone())));

    let input_state = cx.use_hook(|_| Rc::new(RefCell::new(String::new())));
    let input_state_b = input_state.clone();

    let shared_state_len = shared_state.borrow().len();
    let on_text_changed = |_: &str| {};

    cx.render(rsx! (
        input {
            r#type: "text",
            onchange: move |ev| { *input_state.borrow_mut() = ev.data.value.clone() }
        }
        button {
            onclick: move |_| {
                shared_state.borrow_mut().push(TodoState {
                    text: input_state_b.borrow().clone(),
                    done: false,
                });
                eprintln!("handling update: {:?}", shared_state.borrow());
            },
            "add"
        }
        shared_state.borrow().iter().map(|item| rsx!( TodoItem {
            text: &item.text,
            done: item.done,
            on_text_changed: &mut on_text_changed,
        } ))
    ))
}

#[derive(Clone)]
struct SharedTodoState(Rc<RefCell<Vec<TodoState>>>);
#[derive(Debug)]
struct TodoState {
    text: String,
    done: bool,
}

#[derive(PartialEq, dioxus::prelude::Props)]
struct TodoItemProps<'a, 'c> {
    text: &'a str,
    done: bool,
    on_text_changed: EventHandler<'c, &'c str>,
}

fn TodoItem<'a, 'c>(cx: Scope<TodoItemProps<'a, 'c>>) -> Element<'a> {
    use dioxus::prelude::*;
    eprintln!("todo item");

    let times = cx.use_hook(|_| 0_i64);
    let state = cx.consume_context::<SharedTodoState>().unwrap();

    cx.render(rsx!(
        div {
            input {
                r#type: "checkbox",
                checked: "{cx.props.done}",
            }
            "{cx.props.text}"
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
