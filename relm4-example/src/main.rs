mod counter;

use counter::{Counter, CounterOutput};
use gtk::{
    prelude::{BoxExt, GtkWindowExt},
    traits::OrientableExt, traits::ButtonExt,
};
use relm4::{gtk, factory::FactoryVecDeque};
use relm4::ComponentSender;

struct AppModel {
    counters: FactoryVecDeque<Counter<AppMsg>>,
}

#[derive(Debug)]
enum AppMsg {
    FromCounter(CounterOutput),
    Add,
}

impl From<CounterOutput> for AppMsg {
    fn from(x: CounterOutput) -> Self {
        AppMsg::FromCounter(x)
    }
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
                gtk::Button {
                    set_label: "Add",
                    connect_clicked => AppMsg::Add,
                },

                #[local_ref]
                counter_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                },
            },
        }
    }

    fn init(
        (): Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let mut model = AppModel {
            counters: FactoryVecDeque::new(gtk::Box::default(), sender.input_sender()),
        };

        // Insert the macro code generation here
        let counter_box = model.counters.widget();
        let widgets = view_output!();

        model.counters.guard().push_back(0);

        relm4::ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        msg: Self::Input,
        _sender: relm4::ComponentSender<Self>,
    ) {
        match msg {
            AppMsg::Add => {
                self.counters.guard().push_back(0);
            }
            AppMsg::FromCounter(CounterOutput::MoveUp(index)) => {
                let index = index.current_index();
                let new_index = index + 1;
                if new_index < self.counters.len() {
                    self.counters.guard().move_to(index, new_index);
                }
            }
            AppMsg::FromCounter(CounterOutput::MoveDown(index)) => {
                let index = index.current_index();
                let (new_index, overflown) = index.overflowing_sub(1);
                if !overflown {
                    self.counters.guard().move_to(index, new_index);
                }
            }
            AppMsg::FromCounter(CounterOutput::Delete(index)) => {
                let index = index.current_index();
                self.counters.guard().remove(index);
            }
        }
    }
}

fn main() {
    let app = relm4::RelmApp::new("dafuq.is.this");
    app.run::<AppModel>(());
}
