pub mod lens;
mod todo_item;
mod view_tuple;

use todo_item::TodoState;
use lens::{Ix, LensExt};

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
                    rui::button("Add", move |cx| {
                        let state = &mut cx[app_state];
                        state.items.push(TodoState::new(state.new_item.clone()))
                    })
                )),

                {
                    let items = (0..cx[app_state].items.len()).map(|index| {
                        let lens = AppState::items().zoom(Ix::new(index));
                        let binding = rui::bind(app_state, lens);
                        todo_item::todo_item(binding)
                    }).collect::<Vec<_>>();
                    let view = view_tuple::ViewSlice::new(&items);
                    rui::vstack(view)
                }
            ))
        },
    ));
}
