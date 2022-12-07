use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::{Scope, Element};


#[derive(PartialEq, dioxus::prelude::Props)]
pub struct TodoItemProps {
    pub text: Rc<RefCell<String>>,
    pub done: Rc<RefCell<bool>>,
}

pub fn TodoItem(cx: Scope<TodoItemProps>) -> Element {
    use dioxus::prelude::*;

    let done: &bool = &cx.props.done.borrow();
    let text: &String = &cx.props.text.borrow();
    let is_editing = use_state(&cx, || false);
    let input_state = use_state(&cx, || String::new());
    let input_state_b = input_state.clone();

    cx.render(rsx!(
        div {
            input {
                r#type: "checkbox",
                checked: "{done}",
                onchange: move |ev| { *cx.props.done.borrow_mut() = ev.data.value == "true" }
            }
            if *is_editing.get() {
                rsx!(input {
                    r#type: "text",
                    value: "{text}",
                    onchange: move |ev| { input_state.set(ev.data.value.clone()) }
                })
            } else {
                rsx!("{text}")
            }
            button {
                onclick: move |_| {
                    let editing = *is_editing.get();
                    if editing {
                        *cx.props.text.borrow_mut() = input_state_b.get().clone();
                    }
                    is_editing.set(!editing);
                },
                if *is_editing.get() {
                    rsx!("done")
                } else {
                    rsx!("edit")
                }
            }
        }
    ))
}

