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
    use gtk::glib::once_cell::sync::Lazy;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk4 as gtk;

    #[derive(Default)]
    pub struct Counter {
        value: std::cell::Cell<i64>,
        label: std::cell::RefCell<Option<gtk::Label>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Counter {
        const NAME: &'static str = "MyGtkAppCounter";
        type Type = super::Counter;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for Counter {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> =
                Lazy::new(|| vec![glib::ParamSpecInt64::builder("value").build()]);
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.name() {
                "value" => {
                    let input_number =
                        value.get::<i64>().expect("The value needs to be of type `i64`.");
                    self.value.replace(input_number);
                    match *self.label.borrow() {
                        None => (),
                        Some(ref label) => label.set_label(&format!("{}", input_number)),
                    }
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "value" => self.value.get().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(&obj);
            let layout = obj.upcast_ref::<gtk::Box>();

            let label = gtk::Label::new(Some("0"));
            *self.label.borrow_mut() = Some(label.clone());

            let button_minus = gtk::Button::with_label("-");
            button_minus.connect_clicked(glib::clone!(@strong obj => move |_| {
                let s = obj.property::<i64>("value");
                obj.set_property("value", s - 1);
            }));
            layout.append(&button_minus);

            layout.append(&label);

            let button_plus = gtk::Button::with_label("+");
            button_plus.connect_clicked(glib::clone!(@strong obj => move |_| {
                let s = obj.property::<i64>("value");
                obj.set_property("value", s + 1);
            }));
            layout.append(&button_plus);
        }
    }

    impl WidgetImpl for Counter {}

    impl BoxImpl for Counter {}
}
