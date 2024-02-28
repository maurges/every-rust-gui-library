mod todo_item;

use ribir::prelude::*;

fn main() {
    let app = fn_widget! {
        let todos = vec![
            todo_item::TodoItem {text: "kek".to_owned(), done: false},
            todo_item::TodoItem {text: "cheburek".to_owned(), done: false},
        ];
        let todos = State::value(todos);

        @VScrollBar {
            @Column {
                @{
                    let todos = clone_state(&todos);
                    let input = @Input {};
                    let button = @FilledButton {
                        on_tap: move |_| {
                            let text = std::borrow::Borrow::<str>::borrow($input.text()).to_owned();
                            let new_item = todo_item::TodoItem {text, done: false};
                            $todos.write().push(new_item);
                        },
                        @{ Label::new("add") }
                    };
                    @Row { @$input {} @$button {} }
                }
                @{
                    let todos = clone_state(&todos);
                    pipe!($todos.len()).map(move |len| {
                        @Column { @{
                            (0..len).filter_map(|i| {
                                // I have to use write() here otherwise macro
                                // does some bullshit and I can't do map_writer
                                // below
                                if i >= $todos.write().len() {
                                    None
                                } else {
                                    let proj = todos.map_writer(move |xs| &xs[i], move |xs| &mut xs[i]);
                                    Some(todo_item::TodoWidget::new(proj))
                                }
                            }) }
                        }.widget_build(ctx!())
                    })
                }
                @{
                    let todos1 = clone_state(&todos);
                    let save_button = @FilledButton {
                        on_tap: move |_| {
                            let val = $todos1.len();
                            eprintln!("todos: {:?}", val);
                        },
                        @{ Label::new("save") }
                    };

                    let todos2 = clone_state(&todos);
                    let load_button = @FilledButton {
                        on_tap: move |_| {
                            *$todos2.write() = Vec::new();
                        },
                        @{ Label::new("load") }
                    };

                    @Row { @$save_button {} @$load_button {} }
                }
            }
        }
    };
    App::run(app);
}

fn clone_state<T, S: StateWriter<Value = T>>(s: &S) -> impl StateWriter<Value = T> {
    s.map_writer(|t| t, |t| t)
}
