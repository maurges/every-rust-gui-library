use std::collections::HashMap;

use qmetaobject::{
    qt_base_class, qt_method, QAbstractListModel, QByteArray, QGadget, QMetaType, QModelIndex,
    QObject, QString, QUrl, QVariant, USER_ROLE,
};

#[derive(QGadget, Clone, Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct TodoItem {
    pub text: String,
    pub done: bool,
}

impl TodoItem {
    fn get(&self, role: i32) -> QVariant {
        if role == 0 {
            QString::from(self.text.clone()).into()
        } else if role == 1 {
            self.done.into()
        } else {
            Default::default()
        }
    }
    fn set(&mut self, role: i32, x: QVariant) -> bool {
        let x = x.clone();
        if role == 0 {
            if let Some(text) = QMetaType::from_qvariant(x) {
                self.text = text;
                true
            } else {
                false
            }
        } else if role == 1 {
            if let Some(done) = QMetaType::from_qvariant(x) {
                self.done = done;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn names() -> HashMap<i32, QByteArray> {
        HashMap::from([(0, "text".into()), (1, "done".into())])
    }
}

#[derive(QObject, Default)]
pub struct TodoListModel {
    base: qt_base_class!(trait QAbstractListModel),
    values: Vec<TodoItem>,

    append: qt_method!(fn(&mut self, value: TodoItem)),
    save_items: qt_method!(fn(&self, path: QUrl) -> QString),
    load_items: qt_method!(fn(&mut self, path: QUrl) -> QString),

    make: qt_method!(
        fn make(&self, text: String) -> QVariant {
            TodoItem { text, done: false }.to_qvariant()
        }
    ),
}

impl TodoListModel {
    pub fn insert(&mut self, index: usize, element: TodoItem) {
        (self as &mut dyn QAbstractListModel).begin_insert_rows(index as i32, index as i32);
        self.values.insert(index, element);
        (self as &mut dyn QAbstractListModel).end_insert_rows();
    }
    pub fn append(&mut self, value: TodoItem) {
        let idx = self.values.len();
        self.insert(idx, value);
    }
    pub fn reset_data(&mut self, data: Vec<TodoItem>) {
        (self as &mut dyn QAbstractListModel).begin_reset_model();
        self.values = data;
        (self as &mut dyn QAbstractListModel).end_reset_model();
    }

    pub fn save_items(&mut self, path: QUrl) -> QString {
        let dest: String = QString::from(path).into();
        let dest = dest.strip_prefix("file://").unwrap_or(&dest);
        let r = match std::fs::File::create(dest) {
            Ok(file) => match ron::ser::to_writer(file, &self.values) {
                Ok(()) => "".to_owned(),
                Err(e) => format!("{}", e),
            },
            Err(e) => format!("{}", e),
        };
        r.into()
    }

    pub fn load_items(&mut self, path: QUrl) -> QString {
        let src: String = QString::from(path).into();
        let src = src.strip_prefix("file://").unwrap_or(&src);
        match std::fs::File::open(src) {
            Ok(file) => match ron::de::from_reader(file) {
                Ok(items) => {
                    self.reset_data(items);
                    "".into()
                }
                Err(e) => format!("{}", e).into(),
            },
            Err(e) => format!("{}", e).into(),
        }
    }

    fn checked_index(&self, index: QModelIndex) -> Option<usize> {
        let idx = index.row();
        if idx < 0 || idx >= self.values.len() as i32 {
            None
        } else {
            Some(idx as usize)
        }
    }
}

impl QAbstractListModel for TodoListModel {
    fn row_count(&self) -> i32 {
        self.values.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let idx = match self.checked_index(index) {
            None => return Default::default(),
            Some(i) => i,
        };
        self.values[idx].get(role - USER_ROLE).clone()
    }
    fn set_data(&mut self, index: QModelIndex, value: &QVariant, role: i32) -> bool {
        let role = role - USER_ROLE;
        let idx = match self.checked_index(index) {
            None => return false,
            Some(i) => i,
        };
        let r = self.values[idx].set(role, value.clone());
        if r {
            (self as &mut dyn QAbstractListModel).data_changed(index, index);
        }
        r
    }

    fn role_names(&self) -> HashMap<i32, qmetaobject::QByteArray> {
        TodoItem::names()
            .iter()
            .map(|(i, x)| (i + USER_ROLE, x.clone()))
            .collect()
    }
}
