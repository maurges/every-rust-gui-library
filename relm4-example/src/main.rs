use gtk::{
    prelude::{BoxExt, ButtonExt, GtkWindowExt},
    traits::OrientableExt,
};
use relm4::gtk;
use relm4::{ComponentSender, RelmWidgetExt};

#[tracker::track]
#[derive(Default)]
struct AppModel {
    counter1: isize,
    counter2: isize,
}

#[derive(Debug)]
enum AppMsg {
    Change(isize, usize),
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
                set_orientation: gtk::Orientation::Vertical,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Button {
                        set_label: "-",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::Change(-1, 0))
                        }
                    },

                    gtk::Label {
                        #[track = "model.changed(AppModel::counter1())"]
                        set_label: &format!("{}", model.counter1)
                    },

                    gtk::Button {
                        set_label: "+",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::Change(1, 0))
                        }
                    },
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Button {
                        set_label: "-",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::Change(-1, 1))
                        }
                    },

                    gtk::Label {
                        #[track = "model.changed(AppModel::counter2())"]
                        set_label: &format!("{}", model.counter2)
                    },

                    gtk::Button {
                        set_label: "+",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::Change(1, 1))
                        }
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel {
            counter1: init,
            counter2: init,
            tracker: Default::default(),
        };

        // Insert the macro code generation here
        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        AppMsg::Change(delta, index): Self::Input,
        _sender: relm4::ComponentSender<Self>,
    ) {
        self.reset();
        if index == 0 {
            self.update_counter1(|x| *x += delta);
        } else if index == 1 {
            self.update_counter2(|x| *x += delta);
        }
    }
}

fn main() {
    let app = relm4::RelmApp::new("dafuq.is.this");
    app.run::<AppModel>(0);
}
