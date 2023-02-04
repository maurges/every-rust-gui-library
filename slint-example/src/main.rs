use std::cell::RefCell;

use slint::Model;

slint::include_modules!();

struct VecModel<T> {
    items: RefCell<Vec<T>>,
    notify: slint::ModelNotify,
}

impl<T: Clone + 'static> Model for VecModel<T> {
    type Data = T;

    fn row_count(&self) -> usize {
        self.items.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        match self.items.borrow().get(row) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

    fn model_tracker(&self) -> &dyn slint::ModelTracker {
        &self.notify
    }

    fn set_row_data(&self, row: usize, data: Self::Data) {
        let mut items = self.items.borrow_mut();
        if row < items.len() {
            items[row] = data;
            self.notify.row_changed(row);
        } else if row == items.len() {
            items.push(data);
            self.notify.row_added(row, 1);
        }
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl<T: Clone + 'static> VecModel<T> {
    fn rc_from_vec(xs: Vec<T>) -> slint::ModelRc<T> {
        let model = Self {
            items: RefCell::new(xs),
            notify: slint::ModelNotify::default(),
        };
        slint::ModelRc::new(model)
    }
}

fn main() {
    let window = UiMain::new();
    let globals = window.global::<Globals>();

    globals.on_empty_items(|| {
        VecModel::rc_from_vec(Vec::new())
    });

    globals.on_init_world(|| {
        RealWorld {
            use_count: 0,
        }
    });

    globals.on_push_item(|text, items, mut rw| {
        let x = TodoState {
            text,
            done: false,
        };
        items.set_row_data(items.row_count(), x);
        rw.use_count += 1;
        rw
    });

    globals.on_load_items(|mut rw| {
        rw.use_count += 1;
        LoadType {
            rw,
            value: VecModel::rc_from_vec(Vec::new()),
        }
    });

    globals.on_save_items(|model| {
        let items = model.iter().collect::<Vec<_>>();
        eprintln!("saving: {:?}", items);
    });

    window.run()
}
