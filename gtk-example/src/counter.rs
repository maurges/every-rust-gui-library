use gtk::glib;
use gtk4 as gtk;

glib::wrapper! {
    pub struct Counter(ObjectSubclass<imp::Counter>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Counter {
    pub fn new() -> Self {
        glib::Object::builder().build().unwrap()
    }
}

mod imp {
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk4 as gtk;

    #[derive(Default)]
    pub struct Counter;

    #[glib::object_subclass]
    impl ObjectSubclass for Counter {
        const NAME: &'static str = "MyGtkAppCounter";
        type Type = super::Counter;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for Counter {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(&obj);
            let state = std::rc::Rc::new(std::cell::RefCell::new(0isize));

            let layout = obj.upcast_ref::<gtk::Box>();

            let label = gtk::Label::new(Some("0"));

            let button_minus = gtk::Button::with_label("-");
            button_minus.connect_clicked(glib::clone!(@strong state, @weak label => move |_| {
                let mut s = state.borrow_mut();
                *s -= 1;
                label.set_label(&format!("{}", *s));
            }));
            layout.append(&button_minus);

            layout.append(&label);

            let button_plus = gtk::Button::with_label("+");
            button_plus.connect_clicked(glib::clone!(@strong state, @weak label => move |_| {
                let mut s = state.borrow_mut();
                *s += 1;
                label.set_label(&format!("{}", *s));
            }));
            layout.append(&button_plus);
        }
    }

    impl WidgetImpl for Counter {}

    impl BoxImpl for Counter {}
}
