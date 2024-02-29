mod todo_item;

use ribir::prelude::*;

fn main() {
    let app = fn_widget! {
        let todos = Vec::new();
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
                            (0..len).map(|i| {
                                let proj = todos.map_writer(move |xs| &xs[i], move |xs| &mut xs[i]);
                                todo_item::TodoWidget::new(proj)
                            }) }
                        }.widget_build(ctx!())
                    })
                }
                @{
                    let todos1 = clone_state(&todos);
                    let save_button = @FilledButton {
                        on_tap: move |_| {
                            save(&$todos1);
                        },
                        @{ Label::new("save") }
                    };

                    let todos2 = clone_state(&todos);
                    let load_button = @FilledButton {
                        on_tap: move |_| {
                            let items = load().unwrap();
                            *$todos2.write() = items;
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

fn save(items: &[todo_item::TodoItem]) {
    if let Some(path) = rfd::FileDialog::new().save_file() {
        match std::fs::File::create(&path) {
            Ok(file) => match ron::ser::to_writer(file, items) {
                Ok(()) => (),
                Err(e) => eprintln!("{}", e),
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn load() -> Option<Vec<todo_item::TodoItem>> {
    let path = rfd::FileDialog::new().pick_file()?;
    match std::fs::File::open(path) {
        Err(e) => { eprintln!("{}", e); None }
        Ok(file) => match ron::de::from_reader(file) {
            Ok(items) => Some(items),
            Err(e) => { eprintln!("{}", e); None }
        }
    }
}
