use relm4::gtk;
use gtk::{prelude::{BoxExt, ButtonExt, GtkWindowExt}, traits::OrientableExt};
use relm4::{RelmWidgetExt, ComponentSender};

#[derive(Default)]
struct AppModel {
    counter: isize,
}

#[derive(Debug)]
enum AppMsg {
    Change(isize),
}

#[relm4::component]
impl relm4::SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = isize;
    type Widgets = AppWidgets;

    view! {
        gtk::Window {
            set_title: Some("hello gtk"),
            set_default_width: 640,
            set_default_height: 480,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "-",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Change(-1))
                    }
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("{}", model.counter)
                },

                gtk::Button {
                    set_label: "+",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Change(1))
                    }
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { counter: init };

        // Insert the macro code generation here
        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, AppMsg::Change(delta): Self::Input, _sender: relm4::ComponentSender<Self>) {
        self.counter += delta;
    }
}

fn main() {
    let app = relm4::RelmApp::new("dafuq.is.this");
    app.run::<AppModel>(0);
}
