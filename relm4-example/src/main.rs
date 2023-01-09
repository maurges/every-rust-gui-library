mod todo_item;

use gtk::{
    prelude::{BoxExt, GtkWindowExt},
    traits::OrientableExt,
};
use relm4::ComponentSender;
use relm4::{
    factory::FactoryVecDeque,
    gtk::{self, prelude::EntryBufferExtManual, traits::ButtonExt},
};
use todo_item::{TodoItem, TodoState};

struct AppModel {
    todo_items: FactoryVecDeque<TodoItem<AppMsg>>,
    entry_buffer: gtk::EntryBuffer,
}

#[derive(Debug)]
enum AppMsg {
    Add,
    Save,
    Load,
}

#[relm4::component]
impl relm4::SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = ();
    type Widgets = AppWidgets;

    view! {
        gtk::Window {
            set_title: Some("hello gtk"),
            set_default_width: 640,
            set_default_height: 480,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,

                    gtk::Entry::with_buffer(&model.entry_buffer) {
                    },
                    gtk::Button {
                        set_label: "Add",
                        connect_clicked => AppMsg::Add,
                    },
                },

                #[local_ref]
                todos_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,

                    gtk::Button {
                        set_label: "Save",
                        connect_clicked => AppMsg::Save,
                    },
                    gtk::Button {
                        set_label: "Load",
                        connect_clicked => AppMsg::Load,
                    },
                },
            },
        }
    }

    fn init(
        (): Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel {
            todo_items: FactoryVecDeque::new(gtk::Box::default(), sender.input_sender()),
            entry_buffer: gtk::EntryBuffer::new(None),
        };
        let todos_box = model.todo_items.widget();
        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match msg {
            AppMsg::Add => {
                self.todo_items
                    .guard()
                    .push_back(TodoState::new(self.entry_buffer.text()));
            }
            AppMsg::Save => {
                let items = self
                    .todo_items
                    .iter()
                    .map(|x| x.state.clone())
                    .collect::<Vec<_>>();
                save(&items);
            }
            AppMsg::Load => {
                if let Some(items) = load() {
                    let mut todo_items = self.todo_items.guard();
                    todo_items.clear();
                    for item in items {
                        todo_items.push_back(item);
                    }
                }
            }
        }
    }
}

fn main() {
    let app = relm4::RelmApp::new("dafuq.is.this");
    app.run::<AppModel>(());
}

fn save(items: &[TodoState]) {
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

fn load() -> Option<Vec<TodoState>> {
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
