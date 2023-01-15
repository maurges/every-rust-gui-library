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
                    let view = view_tuple::ViewVec::new(items);
                    rui::vstack(view)
                },

                rui::hstack((
                    rui::button("Save", move |cx| {
                        save(&cx[app_state].items);
                    }),
                    rui::button("Load", move |cx| {
                        if let Some(items) = load() {
                            cx[app_state].items = items;
                        }
                    }),
                )),
            ))
        },
    ));
}

fn save(items: &[TodoState]) {
    if let Some(path) = rfd::FileDialog::new().save_file() {
        match std::fs::File::create(&path) {
            Ok(file) => match ron::ser::to_writer(file, items) {
                Ok(()) => (),
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn load() -> Option<Vec<TodoState>> {
    let path = rfd::FileDialog::new().pick_file()?;
    match std::fs::File::open(path) {
        Err(e) => {
            eprintln!("{}", e);
            None
        }
        Ok(file) => match ron::de::from_reader(file) {
            Ok(items) => Some(items),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        },
    }
}
