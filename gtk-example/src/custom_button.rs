use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

glib::wrapper! {
pub struct CustomButton(ObjectSubclass<imp::ImplCustomButton>)
    @extends gtk::Button, gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomButton {
    pub fn new() -> Self {
        glib::Object::builder().build().unwrap()
    }
    pub fn with_label(label: &str) -> Self {
        glib::Object::builder().property("label", label).build().unwrap()
    }

    pub fn set_label(&self, label: &str) {
        self.set_property("label", label)
    }
}

mod imp {
    use gtk::glib;
    use gtk::glib::once_cell::sync::Lazy;
    use gtk::prelude::{ToValue, ObjectExt};
    use gtk::subclass::prelude::*;
    use gtk4 as gtk;

    // Object holding the state
    #[derive(Default)]
    pub struct ImplCustomButton {
        pressed: std::cell::Cell<usize>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ImplCustomButton {
        const NAME: &'static str = "MyGtkAppCustomButton";
        type Type = super::CustomButton;
        type ParentType = gtk::Button;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for ImplCustomButton {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> =
                Lazy::new(|| vec![glib::ParamSpecUInt::builder("number").build()]);
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.name() {
                "number" => {
                    let input_number =
                        value.get::<u32>().expect("The value needs to be of type `u32`.") as usize;
                    self.pressed.replace(input_number);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "number" => (self.pressed.get() as u32).to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.bind_property("number", obj, "label")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for ImplCustomButton {}

    // Trait shared by all buttons
    impl ButtonImpl for ImplCustomButton {
        fn clicked(&self, obj: &Self::Type) {
            self.pressed.set(self.pressed.get() + 1);
            obj.set_property("number", self.pressed.get() as u32);
        }
    }
}
