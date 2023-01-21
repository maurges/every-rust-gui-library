mod custom_button;

use custom_button::CustomButton;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, glib};

fn build_ui(application: &Application) {
    let state = std::rc::Rc::new(std::cell::RefCell::new(0isize));

    let window = ApplicationWindow::builder()
        .application(application)
        .title("First GTK Program")
        .default_width(350)
        .default_height(70)
        .build();

    let layout = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    window.set_child(Some(&layout));

    let label = gtk::Label::new(Some("0"));

    let button_minus = CustomButton::with_label("-");
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

    window.show();
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(build_ui);

    application.run();
}

