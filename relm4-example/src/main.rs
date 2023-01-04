use gtk4::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

#[derive(Default)]
struct AppModel {
    counter: isize,
}

enum AppMsg {
    Change(isize),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Change(delta) => {
                self.counter = self.counter + delta;
            }
        }
        true
    }
}

struct AppWidgets {
    label: gtk4::Label,
}

impl SimpleComponent for AppModel {

}

/*
#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk4::ApplicationWindow {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,
            set_child = Some(&gtk4::Box) {
                set_orientation: gtk4::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk4::Button {
                    set_label: "Increment",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Change(1));
                    },
                },
                append = &gtk4::Button {
                    set_label: "Decrement",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Change(-1));
                    },
                },
                append = &gtk4::Label {
                    set_margin_all: 5,
                    set_label: watch! { &format!("Counter: {}", model.counter) },
                }
            },
        }
    }
}
*/

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model);
    // app.run();
}
