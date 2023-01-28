mod counter;
mod custom_button;
mod integer_object;

use custom_button::CustomButton;

use gtk::{gio, prelude::*};
use gtk::{glib, Application, ApplicationWindow, Button};
use gtk4 as gtk;

fn build_ui(application: &Application) {
    let state = std::rc::Rc::new(std::cell::RefCell::new(0isize));
    // state for list of counters
    let mut vec_model = gio::ListStore::new(integer_object::IntegerObject::static_type());
    vec_model.extend((0..10).map(integer_object::IntegerObject::new));

    let window = ApplicationWindow::builder()
        .application(application)
        .title("First GTK Program")
        .default_width(350)
        .default_height(70)
        .build();

    let layout = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    window.set_child(Some(&layout));

    let label = gtk::Label::new(Some("0"));

    let button_minus = CustomButton::new();
    button_minus.connect_clicked(glib::clone!(@strong state, @weak label => move |_| {
        let mut s = state.borrow_mut();
        *s -= 1;
        label.set_label(&format!("{}", *s));
    }));
    layout.append(&button_minus);

    layout.append(&label);

    let button_plus = Button::with_label("+");
    button_plus.connect_clicked(glib::clone!(@strong state, @weak label => move |_| {
        let mut s = state.borrow_mut();
        *s += 1;
        label.set_label(&format!("{}", *s));
    }));
    layout.append(&button_plus);

    let counter1 = counter::Counter::new();
    let counter2 = counter::Counter::new();
    counter1
        .bind_property("value", &counter2, "value")
        .flags(glib::BindingFlags::BIDIRECTIONAL)
        .build();
    layout.append(&counter1);
    layout.append(&counter2);

    // list of counters
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let counter = counter::Counter::new();
        list_item.set_child(Some(&counter));
    });
    factory.connect_bind(move |_, list_item| {
        let integer_object = list_item
            .item()
            .expect("Item should be set")
            .downcast::<integer_object::IntegerObject>()
            .unwrap();
        let counter = list_item
            .child()
            .expect("Child should be set")
            .downcast::<counter::Counter>()
            .unwrap();
        counter
            .bind_property("value", &integer_object, "value")
            .flags(glib::BindingFlags::BIDIRECTIONAL | glib::BindingFlags::SYNC_CREATE)
            .build();
    });

    let selection_model = gtk::NoSelection::new(Some(&vec_model));
    let list_view = gtk::ListView::new(Some(&selection_model), Some(&factory));
    layout.append(&list_view);

    // button to print list model
    let button_print = Button::with_label("?");
    button_print.connect_clicked(glib::clone!(@weak vec_model => move |_| {
        let len = vec_model.n_items();
        println!("Items in model: {}", len);
        for i in 0..len {
            let obj = vec_model.item(i).unwrap();
            let intobj = obj.downcast_ref::<integer_object::IntegerObject>().unwrap();
            let value = intobj.property::<i64>("value");
            println!("{}", value);
        }

    }));
    layout.append(&button_print);

    window.show();
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(build_ui);

    application.run();
}
