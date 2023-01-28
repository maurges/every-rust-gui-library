use gtk::glib;
use gtk4 as gtk;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TodoState {
    pub text: String,
    pub done: bool,
}

glib::wrapper! {
    pub struct TodoStateObject(ObjectSubclass<imp::TodoStateObject>);
}

impl TodoStateObject {
    pub fn new(text: String) -> Self {
        glib::Object::builder().property("text", text).build().unwrap()
    }

    pub fn from_state(s: TodoState) -> Self {
        glib::Object::builder()
            .property("text", s.text)
            .property("done", s.done)
            .build()
            .unwrap()
    }
}

glib::wrapper! {
    pub struct TodoItem(ObjectSubclass<imp::TodoItem>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl TodoItem {
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

    use super::TodoState;

    #[derive(Default)]
    pub struct TodoStateObject {
        value: std::cell::RefCell<TodoState>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TodoStateObject {
        const NAME: &'static str = "MyGtkAppTodoStateObject";
        type Type = super::TodoStateObject;
    }

    impl ObjectImpl for TodoStateObject {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecString::builder("text").build(),
                    glib::ParamSpecBoolean::builder("done").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "text" => self.value.borrow().text.to_value(),
                "done" => self.value.borrow().done.to_value(),
                n => panic!("incorrect property access: {}", n),
            }
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "text" => {
                    self.value.borrow_mut().text = value
                        .get::<&str>()
                        .expect("Value needs to be string")
                        .to_owned()
                }
                "done" => self.value.borrow_mut().done = value.get().expect("Value needs to be bool"),
                n => panic!("incorrect property access: {}", n),
            }
        }
    }

    #[derive(Default)]
    pub struct TodoItem {
        state: std::cell::RefCell<TodoState>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TodoItem {
        const NAME: &'static str = "MyGtkAppTodoItem";
        type Type = super::TodoItem;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for TodoItem {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecString::builder("text").build(),
                    glib::ParamSpecBoolean::builder("done").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "text" => self.state.borrow().text.to_value(),
                "done" => self.state.borrow().done.to_value(),
                n => panic!("incorrect property access: {}", n),
            }
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "text" => {
                    self.state.borrow_mut().text = value
                        .get::<&str>()
                        .expect("Value needs to be string")
                        .to_owned()
                }
                "done" => self.state.borrow_mut().done = value.get().expect("Value needs to be bool"),
                n => panic!("incorrect property access: {}", n),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(&obj);
            let layout = obj.upcast_ref::<gtk::Box>();

            let checkbox = gtk::CheckButton::new();
            checkbox
                .bind_property("active", obj, "done")
                .flags(glib::BindingFlags::BIDIRECTIONAL | glib::BindingFlags::SYNC_CREATE)
                .build();
            layout.append(&checkbox);

            let label = gtk::Label::new(None);
            label
                .bind_property("label", obj, "text")
                .flags(glib::BindingFlags::BIDIRECTIONAL | glib::BindingFlags::SYNC_CREATE)
                .build();
            layout.append(&label);

            let edit_buffer = gtk::EntryBuffer::new(None);
            edit_buffer
                .bind_property("text", obj, "text")
                .flags(glib::BindingFlags::BIDIRECTIONAL | glib::BindingFlags::SYNC_CREATE)
                .build();
            let edit_label = gtk::Entry::with_buffer(&edit_buffer);
            edit_label.hide();
            layout.append(&edit_label);

            let edit_button = gtk::Button::with_label("Edit");
            let editing = std::rc::Rc::new(std::cell::Cell::new(false));
            edit_button.connect_clicked(glib::clone!(@weak label, @weak edit_label => move |this| {
                let is_editing = !editing.get();
                editing.set(is_editing);
                if is_editing {
                    this.set_label("Done");
                    label.hide();
                    edit_label.show();
                } else {
                    this.set_label("Edit");
                    edit_label.hide();
                    label.show();
                }
            }));
            layout.append(&edit_button);
        }
    }

    impl WidgetImpl for TodoItem {}

    impl BoxImpl for TodoItem {}
}
