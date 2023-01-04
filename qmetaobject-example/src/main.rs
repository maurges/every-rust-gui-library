#![allow(non_snake_case)]
mod todo_list_model;

use cstr::cstr;
use qmetaobject::{qml_register_type, qrc, QmlEngine};
use todo_list_model::TodoListModel;

qrc!(gui_resource,
    "qml" as "qml" {
        "main.qml",
        "TodoItem.qml",
    }
);
fn main() {
    gui_resource();
    let qml_module = cstr!("men.morj.qmetaobject");
    qml_register_type::<TodoListModel>(qml_module, 1, 0, cstr!("TodoListModel"));
    // Create a QML engine from rust
    let mut engine = QmlEngine::new();
    // (Here the QML code is inline, but one can also load from a file)
    engine.load_file("qrc:/qml/main.qml".into());
    engine.exec();
}
