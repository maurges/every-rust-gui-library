use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    let state = std::rc::Rc::new(std::cell::RefCell::new(0isize));

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();

        let layout = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        window.set_child(Some(&layout));

        let label = std::rc::Rc::new(gtk::Label::new(Some("0")));

        let button_minus = Button::with_label("-");
        let state_minus = state.clone();
        let label_minus = label.clone();
        button_minus.connect_clicked(move |_| {
            let mut s = state_minus.borrow_mut();
            *s -= 1;
            label_minus.set_label(&format!("{}", *s));
        });
        layout.append(&button_minus);

        layout.append(label.as_ref());

        let button_plus = Button::with_label("+");
        let state_plus = state.clone();
        let label_plus = label.clone();
        button_plus.connect_clicked(move |_| {
            let mut s = state_plus.borrow_mut();
            *s += 1;
            label_plus.set_label(&format!("{}", *s));
        });
        layout.append(&button_plus);

        window.show();
    });

    application.run();
}

