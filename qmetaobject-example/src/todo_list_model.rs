#![allow(dead_code)]

use std::collections::HashMap;

use qmetaobject::{
    qt_base_class, QAbstractListModel, QModelIndex, QObject, QVariant, USER_ROLE, qt_method, QGadget, QString, QByteArray, QVariantMap, QMetaType,
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
                eprintln!("set text to {}", text);
                self.text = text;
                true
            } else {
                false
            }
        } else if role == 1 {
            if let Some(done) = QMetaType::from_qvariant(x) {
                eprintln!("set done to {}", done);
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
        HashMap::from([
            (0, "modelText".into()),
            (1, "modelDone".into()),
        ])
    }
}

#[derive(QObject, Default)]
pub struct TodoListModel {
    base: qt_base_class!(trait QAbstractListModel),
    values: Vec<TodoItem>,

    append: qt_method!(fn (&mut self, value: TodoItem)),
    get: qt_method!(fn (&self, index: usize) -> QVariantMap),
    setProperty: qt_method!(fn (&mut self, index: usize, prop: QByteArray, value: QVariant)),

    len: qt_method!(fn len(&self) -> i32 {
        self.row_count()
    }),

    make: qt_method!(fn make(&self, text: String) -> QVariant {
        TodoItem {
            text,
            done: false,
        }.to_qvariant()
    }),
}

impl TodoListModel {
    pub fn insert(&mut self, index: usize, element: TodoItem) {
        (self as &mut dyn QAbstractListModel).begin_insert_rows(index as i32, index as i32);
        self.values.insert(index, element);
        (self as &mut dyn QAbstractListModel).end_insert_rows();
    }
    pub fn append(&mut self, value: TodoItem) {
        eprintln!("append: {:?}", value);
        let idx = self.values.len();
        self.insert(idx, value);
    }
    pub fn remove(&mut self, index: usize) {
        (self as &mut dyn QAbstractListModel).begin_remove_rows(index as i32, index as i32);
        self.values.remove(index);
        (self as &mut dyn QAbstractListModel).end_remove_rows();
    }
    pub fn change_line(&mut self, index: usize, value: TodoItem) {
        self.values[index] = value;
        let idx = (self as &mut dyn QAbstractListModel).row_index(index as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx, idx);
    }
    pub fn reset_data(&mut self, data: Vec<TodoItem>) {
        (self as &mut dyn QAbstractListModel).begin_reset_model();
        self.values = data;
        (self as &mut dyn QAbstractListModel).end_reset_model();
    }
    /// Returns an iterator over the items in the model
    pub fn iter(&self) -> impl Iterator<Item = &TodoItem> {
        self.values.iter()
    }

    pub fn get(&self, index: usize) -> QVariantMap {
        let mut r: QVariantMap = Default::default();
        r.insert("modelText".into(), QString::from(self.values[index].text.clone()).into());
        r.insert("modelDone".into(), self.values[index].done.into());
        r
    }
    pub fn setProperty(&mut self, index: usize, prop: QByteArray, value: QVariant) {
        let mb_role = TodoItem::names().iter()
            .find_map(|(k, ref v)| if *v == &prop { Some(*k) } else { None });
        if let Some(role) = mb_role {
            self.values[index].set(role, value);
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
        eprintln!("setting data");
        let idx = match self.checked_index(index) {
            None => return false,
            Some(i) => i,
        };
        eprintln!("at index {}", idx);
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