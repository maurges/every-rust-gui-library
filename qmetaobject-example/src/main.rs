use cstr::cstr;
use qmetaobject::prelude::*;

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
    // Register the `Greeter` struct to QML
    qml_register_type::<Greeter>(cstr!("Greeter"), 1, 0, cstr!("Greeter"));
    qml_register_type::<Counter>(cstr!("Greeter"), 1, 0, cstr!("Counter"));
    // Create a QML engine from rust
    let mut engine = QmlEngine::new();
    // (Here the QML code is inline, but one can also load from a file)
    engine.load_data(r#"
        import QtQuick 2.6
        import QtQuick.Controls 2.6
        import QtQuick.Window 2.0
        // Import our Rust classes
        import Greeter 1.0

        Window {
            visible: true
            // Instantiate the rust struct
            Greeter {
                id: greeter;
                // Set a property
                name: "World"
            }
            Counter {
                id: counter
            }
            Column {
                Text {
                    // Call a method
                    text: greeter.compute_greetings("hello")
                }

                Row {
                    Button {
                        text: "-"
                        onClicked: { counter.value -= 1 }
                    }
                    Text {
                        text: counter.value
                    }
                    Button {
                        text: "+"
                        onClicked: { counter.value += 1 }
                    }
                }
            }
        }
    "#.into());
    engine.exec();
}
