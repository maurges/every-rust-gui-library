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
                    pipe!(*$editing).map(move |is_editing| {
                        if is_editing {
                            let input = @Input {};
                            $input.write().set_text(&$todo_item.text);
                            watch!($todo_item.text.clone()).subscribe(move |text| $input.write().set_text(&text));

                            let button = @FilledButton {
                                on_tap: { move |_| {
                                    *$editing.write() = false;
                                    let text = std::borrow::Borrow::<str>::borrow($input.text()).to_owned();
                                    $todo_item.write().text = text;
                                } },
                                @{ Label::new("done") }
                            };

                            @Row {
                                @$input {}
                                @$button {}
                            }.widget_build(ctx!())

                        } else {
                            let button = @FilledButton {
                                on_tap: { move |_| {
                                    *$editing.write() = true;
                                } },
                                @{ Label::new("edit") }
                            };

                            @Row {
                                @Text {
                                    text: pipe!($todo_item.text.clone())
                                }
                                @$button {}
                            }.widget_build(ctx!())
                        }
                    })
                }
            }
        }
    }
}
