use slint::Model;

slint::include_modules!();

fn model_of_slice<T: Clone + 'static>(xs: &[T]) -> slint::ModelRc<T> {
    let model = slint::VecModel::from_slice(xs);
    slint::ModelRc::new(model)
}

fn main() {
    let window = UiMain::new();
    let globals = window.global::<Globals>();

    /*
    globals.on_empty_model(|| {
        eprintln!("make model");
        let model = slint::VecModel::from_slice(&[
        ]);
        slint::ModelRc::new(model)
    });
    */

    globals.on_init_world(|| {
        RealWorld {
            use_count: 0,
        }
    });

    globals.on_push_item(|text, items| {
        let mut items = items.iter().collect::<Vec<_>>();
        items.push(TodoState {
            text,
            done: false,
        });
        model_of_slice(&items)
    });

    globals.on_load_items(|mut rw| {
        rw.use_count += 1;
        LoadType {
            rw,
            value: model_of_slice(&[]),
        }
    });

    globals.on_save_items(|model| {
        let items = model.iter().collect::<Vec<_>>();
        eprintln!("saving: {:?}", items);
    });

    window.run()
}
