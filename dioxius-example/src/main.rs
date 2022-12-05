#![allow(non_snake_case)]
use dioxus::prelude::{Scope, Element};

fn main() {
    dioxus::desktop::launch_with_props(app, 0, |x| x);
}

fn app(cx: Scope<i32>) -> Element {
    use dioxus::prelude::*;

    let is_done_source = true;
    let is_done = use_state(&cx, || is_done_source);

    cx.render(rsx! (
        div { "Hello, world!" }
        TodoItem {
            text: "test".to_owned(),
            done: **is_done,
        }
    ))
}

#[derive(PartialEq, dioxus::prelude::Props)]
struct TodoItemProps {
    text: String,
    done: bool,
}

fn TodoItem(cx: Scope<TodoItemProps>) -> Element {
    use dioxus::prelude::*;

    let times = cx.use_hook(|_| 0_i64);
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
