use ribir::prelude::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TodoItem {
    pub text: String,
    pub done: bool,
}

pub struct TodoWidget<S> {
    todo_item: S,
    editing: State<bool>,
}

impl<S: StateWriter<Value = TodoItem>> TodoWidget<S> {
    pub fn new(todo_item: S) -> Self {
        Self {
            todo_item,
            editing: State::value(false),
        }
    }
}

impl<S: StateWriter<Value = TodoItem>> Compose for TodoWidget<S> {
    fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
        fn_widget! {
            let todo_item = &$this.todo_item;
            @Row {
                @Checkbox {
                    checked: $todo_item.done,
                }
                @{
                    let editing = &$this.editing;
                    if *$editing {
                        let input = @Input {};
                        $input.write().set_text(&$todo_item.text);
                        @$input {}.widget_build(ctx!())
                    } else {
                        @Text {
                            text: pipe!($todo_item.text.clone())
                        }.widget_build(ctx!())
                    }
                }
                @FilledButton {
                    on_tap: { let editing = &$this.editing; move |_| {
                        let val: bool = *$editing;
                        eprintln!("callback: {}", val);
                        *$editing.write() = !val;
                    } },
                    @{
                        let editing = &$this.editing;
                        let text = if *$editing { "done" } else { "edit" };
                        Label::new(text)
                    }
                }
            }
        }
    }
}
