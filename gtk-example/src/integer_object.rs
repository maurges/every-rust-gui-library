use gtk4 as gtk;
use gtk::glib;

glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl IntegerObject {
    pub fn new(value: i64) -> Self {
        glib::Object::builder().property("value", value).build().unwrap()
    }
}

mod imp {
    use gtk::glib::once_cell::sync::Lazy;
    use gtk::prelude::ToValue;
    use gtk4 as gtk;
    use gtk::glib;
    use gtk::subclass::prelude::*;

    #[derive(Default)]
    pub struct IntegerObject {
        pub value: std::cell::Cell<i64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IntegerObject {
        const NAME: &'static str = "MyGtkAppIntegerObject";
        type Type = super::IntegerObject;
    }

    impl ObjectImpl for IntegerObject {
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
    }
}
