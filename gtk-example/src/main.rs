mod counter;
mod custom_button;
mod integer_object;

mod todo_item;

use gtk::{gio, prelude::*};
use gtk::{glib, Application, ApplicationWindow, Button};
use gtk4 as gtk;

fn build_ui(application: &Application) {
    let todos = gio::ListStore::new(todo_item::TodoStateObject::static_type());

    let window = ApplicationWindow::builder()
        .application(application)
        .title("First GTK Program")
        .default_width(350)
        .default_height(70)
        .build();

    let layout = gtk::Box::new(gtk::Orientation::Vertical, 5);
    window.set_child(Some(&layout));

    let add_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    layout.append(&add_box);
    let add_buffer = gtk::EntryBuffer::new(None);
    let add_label = gtk::Entry::with_buffer(&add_buffer);
    add_box.append(&add_label);
    let add_button = gtk::Button::with_label("Add");
    add_button.connect_clicked(glib::clone!(@weak add_buffer, @weak todos => move |_| {
        let text = add_buffer.text();
        println!("adding text: {}", text);
        todos.extend_from_slice(&[todo_item::TodoStateObject::new(text)]);
    }));
    add_box.append(&add_button);

    // list of todos
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let item = todo_item::TodoItem::new();
        list_item.set_child(Some(&item));
    });
    factory.connect_bind(move |_, list_item| {
        let state_object = list_item
            .item()
            .expect("Item should be set")
            .downcast::<todo_item::TodoStateObject>()
            .unwrap();
        let view = list_item
            .child()
            .expect("Child should be set")
            .downcast::<todo_item::TodoItem>()
            .unwrap();
        state_object
            .bind_property("text", &view, "text")
            .flags(glib::BindingFlags::BIDIRECTIONAL | glib::BindingFlags::SYNC_CREATE)
            .build();
        state_object
            .bind_property("done", &view, "done")
            .flags(glib::BindingFlags::BIDIRECTIONAL | glib::BindingFlags::SYNC_CREATE)
            .build();
    });
    let selection_model = gtk::NoSelection::new(Some(&todos));
    let list_view = gtk::ListView::new(Some(&selection_model), Some(&factory));
    layout.append(&list_view);

    // button to print list model
    let button_print = Button::with_label("?");
    button_print.connect_clicked(glib::clone!(@weak todos => move |_| {
        let len = todos.n_items();
        println!("Items in model: {}", len);
        for i in 0..len {
            let obj = todos.item(i).unwrap();
            let done = obj.property::<bool>("done");
            let text = obj.property::<String>("text");
            println!("{}, {}", done, text);
        }

    }));
    layout.append(&button_print);

    let save_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    layout.append(&save_box);
    let save_button = gtk::Button::with_label("Save");
    save_button.connect_clicked(glib::clone!(@weak todos => move |_| {
        let mut vec = Vec::new();
        let len = todos.n_items();
        for i in 0..len {
            let obj = todos.item(i).unwrap();
            let done = obj.property::<bool>("done");
            let text = obj.property::<String>("text");
            vec.push(todo_item::TodoState { done, text });
        }
        save(&vec);
    }));
    save_box.append(&save_button);
    let load_button = gtk::Button::with_label("Load");
    load_button.connect_clicked(glib::clone!(@weak todos => move |_| {
        if let Some(items) = load() {
            let mut todos = todos;
            todos.remove_all();
            todos.extend(items.into_iter().map(todo_item::TodoStateObject::from_state));
        }
    }));
    save_box.append(&load_button);

    window.show();
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(build_ui);

    application.run();
}

fn save(items: &[todo_item::TodoState]) {
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

fn load() -> Option<Vec<todo_item::TodoState>> {
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
