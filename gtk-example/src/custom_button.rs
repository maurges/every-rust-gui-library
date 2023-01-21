use gtk::glib;
use gtk4 as gtk;

glib::wrapper! {
pub struct CustomButton(ObjectSubclass<imp::ImplCustomButton>)
    @extends gtk::Button, gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomButton {
    pub fn with_label(label: &str) -> Self {
        glib::Object::builder().property("label", label).build().unwrap()
    }
}

mod imp {
    use gtk::glib;
    use gtk::subclass::prelude::*;
    use gtk4 as gtk;

    // Object holding the state
    #[derive(Default)]
    pub struct ImplCustomButton;

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ImplCustomButton {
        const NAME: &'static str = "MyGtkAppCustomButton";
        type Type = super::CustomButton;
        type ParentType = gtk::Button;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for ImplCustomButton {}

    // Trait shared by all widgets
    impl WidgetImpl for ImplCustomButton {}

    // Trait shared by all buttons
    impl ButtonImpl for ImplCustomButton {}
}
