use cstr::cstr;
use qmetaobject::{QObject, QString, QmlEngine, qml_register_type, qt_base_class, qt_method, qt_property, qt_signal, qrc};

qrc!(gui_resource,
    "qml" as "qml" {
        "main.qml",
        "TodoItem.qml",
    }
);

// The `QObject` custom derive macro allows to expose a class to Qt and QML
#[derive(QObject,Default)]
struct Greeter {
    // Specify the base class with the qt_base_class macro
    base: qt_base_class!(trait QObject),
    // Declare `name` as a property usable from Qt
    name: qt_property!(QString; NOTIFY name_changed),
    // Declare a signal
    name_changed: qt_signal!(),
    // And even a slot
    compute_greetings: qt_method!(fn compute_greetings(&self, verb: String) -> QString {
        format!("{} {}", verb, self.name.to_string()).into()
    })
}

#[derive(QObject, Default)]
struct Counter {
    base: qt_base_class!(trait QObject),
    value: qt_property!(i32; NOTIFY value_changed),
    value_changed: qt_signal!(),
}

fn main() {
    gui_resource();
    // Register the `Greeter` struct to QML
    qml_register_type::<Greeter>(cstr!("men.morj.qmetaobject"), 1, 0, cstr!("Greeter"));
    qml_register_type::<Counter>(cstr!("men.morj.qmetaobject"), 1, 0, cstr!("Counter"));
    qml_register_type::<Counter>(cstr!("men.morj.qmetaobject"), 1, 0, cstr!("MyObject"));
    // Create a QML engine from rust
    let mut engine = QmlEngine::new();
    // (Here the QML code is inline, but one can also load from a file)
    engine.load_file("qrc:/qml/main.qml".into());
    engine.exec();
}
