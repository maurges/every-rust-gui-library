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
            let editing = &$this.editing;
            @Row {
                @Checkbox {
                    checked: pipe!($todo_item.done),
                }
                @{
                    pipe!(*$editing).map(move |x| {
                        if x {
                            let input = @Input {};
                            $input.write().set_text(&$todo_item.text);
                            watch!($todo_item.text.clone()).subscribe(move |text| $input.write().set_text(&text));
                            @{ input }.widget_build(ctx!())
                        } else {
                            @Text {
                                text: pipe!($todo_item.text.clone())
                            }.widget_build(ctx!())
                        }
                    })
                }
                @{
                    pipe!(*$editing).map(move |e| {
                        let button = @FilledButton {
                            on_tap: { move |_| {
                                let val = *$editing;
                                *$editing.write() = !val;
                                // if we were editing, write the text to model
                                if val {

                                }
                            } },
                        };
                        if e {
                            @$button {
                                @{ Label::new("done") }
                            }
                        } else {
                            @$button {
                                @{ Label::new("edit") }
                            }
                        }
                    })
                }
            }
        }
    }
}
