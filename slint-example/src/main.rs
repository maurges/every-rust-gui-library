use std::cell::RefCell;

use slint::{Model, ComponentHandle};

mod ui {
    slint::include_modules!();
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TodoState {
    text: String,
    done: bool,
}

impl From<ui::TodoState> for TodoState {
    fn from(x: ui::TodoState) -> Self {
        Self {
            text: x.text.into(),
            done: x.done,
        }
    }
}
impl Into<ui::TodoState> for TodoState {
    fn into(self) -> ui::TodoState {
        ui::TodoState {
            text: self.text.into(),
            done: self.done,
        }
    }
}

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

    fn unerase(model: &slint::ModelRc<T>) -> Option<&Self> {
        model.as_any().downcast_ref()
    }
}

fn main() {
    let window = ui::UiMain::new();
    let globals = window.global::<ui::Globals>();

    globals.on_empty_items(|| {
        VecModel::rc_from_vec(Vec::new())
    });

    globals.on_init_world(|| {
        ui::RealWorld {
            use_count: 0,
        }
    });

    globals.on_push_item(|text, items, mut rw| {
        let x = ui::TodoState {
            text,
            done: false,
        };
        items.set_row_data(items.row_count(), x);
        rw.use_count += 1;
        rw
    });

    globals.on_load_items(|mut rw| {
        rw.use_count += 1;
        let value = match load() {
            Some(items) => items.into_iter().map(|x| x.into()).collect(),
            None => Vec::new(),
        };
        ui::LoadType {
            rw,
            value: VecModel::rc_from_vec(value),
        }
    });

    globals.on_save_items(|model| {
        if let Some(items) = VecModel::unerase(&model) {
            let items: Vec<TodoState> = items.iter().map(|x| x.into()).collect();
            save(&items);
        } else {
            eprintln!("Incorrect model!");
        }
    });

    window.run()
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
