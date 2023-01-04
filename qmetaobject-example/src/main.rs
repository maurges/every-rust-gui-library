#![allow(non_snake_case)]
mod todo_list_model;

use cstr::cstr;
use qmetaobject::{
    qml_register_type, qrc, qt_base_class, qt_method, QObject, QString, QUrl, QmlEngine,
};
use todo_list_model::{TodoListModel, TodoItem};

qrc!(gui_resource,
    "qml" as "qml" {
        "main.qml",
        "TodoItem.qml",
    }
);

#[derive(QObject, Default)]
struct MyObject {
    base: qt_base_class!(trait QObject),
    items: Vec<TodoItem>,
    reading_index: Option<usize>,

    addItem: qt_method!(fn(&mut self, text: String, done: bool)),
    saveItems: qt_method!(fn(&mut self, path: QUrl) -> QString),
    loadItems: qt_method!(fn(&mut self, path: QUrl) -> QString),
    nextItem: qt_method!(fn(&mut self) -> bool),
    nextItemText: qt_method!(fn(&self) -> QString),
    nextItemDone: qt_method!(fn(&self) -> bool),
}

impl MyObject {
    fn addItem(&mut self, text: String, done: bool) {
        eprintln!("add item: {} {}", text, done);
        let item = TodoItem {
            text: text.to_string(),
            done,
        };
        self.items.push(item);
    }

    pub fn saveItems(&mut self, path: QUrl) -> QString {
        let dest: String = QString::from(path).into();
        let dest = dest.strip_prefix("file://").unwrap_or(&dest);
        let r = match std::fs::File::create(dest) {
            Ok(file) => match ron::ser::to_writer(file, &self.items) {
                Ok(()) => "".to_owned(),
                Err(e) => format!("{}", e),
            },
            Err(e) => format!("{}", e),
        };
        self.items = Vec::new();
        eprintln!("saved");
        r.into()
    }

    pub fn loadItems(&mut self, path: QUrl) -> QString {
        let src: String = QString::from(path).into();
        eprintln!("loading {}", src);
        let src = src.strip_prefix("file://").unwrap_or(&src);
        match std::fs::File::open(src) {
            Ok(file) => match ron::de::from_reader(file) {
                Ok(items) => {
                    self.items = items;
                    "".into()
                }
                Err(e) => format!("{}", e).into(),
            },
            Err(e) => format!("{}", e).into(),
        }
    }

    pub fn nextItem(&mut self) -> bool {
        match self.reading_index {
            None if self.items.is_empty() => false,
            None => {
                self.reading_index = Some(0);
                true
            }
            Some(mut i) => {
                i += 1;
                self.reading_index = Some(i);
                if i < self.items.len() {
                    true
                } else {
                    self.items = Vec::new();
                    false
                }
            }
        }
    }

    pub fn nextItemText(&self) -> QString {
        match self.reading_index {
            None => "".into(),
            Some(i) => self.items[i].text.clone().into(),
        }
    }
    pub fn nextItemDone(&self) -> bool {
        match self.reading_index {
            None => false,
            Some(i) => self.items[i].done,
        }
    }
}

fn main() {
    gui_resource();
    let qml_module = cstr!("men.morj.qmetaobject");
    qml_register_type::<MyObject>(qml_module, 1, 0, cstr!("MyObject"));
    qml_register_type::<TodoListModel>(
        qml_module,
        1, 0,
        cstr!("TodoListModel"),
    );
    // Create a QML engine from rust
    let mut engine = QmlEngine::new();
    // (Here the QML code is inline, but one can also load from a file)
    engine.load_file("qrc:/qml/main.qml".into());
    engine.exec();
}
