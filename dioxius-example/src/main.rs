#![allow(non_snake_case)]
use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::{Scope, Element};

fn main() {
    dioxus::desktop::launch_with_props(app, 0, |x| x);
}

fn app(cx: Scope<i32>) -> Element {
    use dioxus::prelude::*;

    let make_shared_state = || Rc::new(RefCell::new(vec![
        TodoState { text: "example".into(), done: true },
        TodoState { text: "example2".into(), done: false },
    ]));
    let shared_state = use_state(&cx, || {
        let state = make_shared_state();
        cx.provide_context(SharedTodoState(state.clone()));
        state
    });

    let input_state = cx.use_hook(|_| Rc::new(RefCell::new(String::new())));
    let input_state_b = input_state.clone();

    let shared_state_len = shared_state.borrow().len();
    let rendered_items = (0..shared_state_len).map(|i|
        rsx!(TodoItem {
            index: i,
        })
    );

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
        rendered_items
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
struct TodoItemProps {
    index: usize,
}

fn TodoItem(cx: Scope<TodoItemProps>) -> Element {
    use dioxus::prelude::*;
    eprintln!("todo item");

    let times = cx.use_hook(|_| 0_i64);
    let state = cx.consume_context::<SharedTodoState>().unwrap();
    let my_state = state.0.borrow();
    let done = my_state[cx.props.index].done;
    let text = &my_state[my_state.len() - 1 - cx.props.index].text;

    cx.render(rsx!(
        div {
            input {
                r#type: "checkbox",
                checked: "{done}",
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
