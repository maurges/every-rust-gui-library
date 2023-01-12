pub mod lens;
mod todo_item;

use todo_item::TodoState;

use rui::Modifiers;

use crate::lens::{Ix, LensExt};

#[derive(Default)]
struct AppState {
    new_item: String,
    items: Vec<TodoState>,
}

lens::make_bind!(crate::AppState, String, new_item);
lens::make_lens!(crate::AppState, Vec<crate::todo_item::TodoState>, items);

fn main() {
    use new_item::Bind as Bind_new_item;
    rui::rui(rui::state(
        || Default::default(),
        |app_state, cx| {
            rui::vstack((
                rui::hstack((
                    rui::text_editor(app_state.new_item()),
                    rui::button("Add", |cx| {
                        let mut state = cx[app_state];
                        state.items.push(TodoState::new(state.new_item))
                    })
                )),

                {
                    let bindings = (0..(cx[app_state].items.len())).map(|i| {
                        rui::bind(app_state, AppState::items().zoom(Ix::new(i)))
                    }).collect();
                    rui::list(bindings, |b| todo_item::todo_item(b))
                }
            ))
        },
    ));
}
