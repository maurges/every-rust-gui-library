mod counter;
mod controls;

use counter::{Counter, CounterOutput};
use gtk::{
    prelude::{BoxExt, GtkWindowExt},
    traits::OrientableExt,
};
use relm4::{gtk, factory::FactoryVecDeque, Component, ComponentController};
use relm4::ComponentSender;

struct AppModel {
    counters: FactoryVecDeque<Counter<AppMsg>>,
}

#[derive(Debug)]
enum AppMsg {
    FromCounter(CounterOutput),
    Add(isize),
    DeleteLast,
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

                controls.widget(),

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

        let controls = controls::Controls::builder().launch(())
            .forward(sender.input_sender(), |x| match x {
                controls::Output::Add(x) => AppMsg::Add(x),
                controls::Output::DeleteLast => AppMsg::DeleteLast,
            });

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
            AppMsg::Add(val) => {
                self.counters.guard().push_back(val);
            }
            AppMsg::DeleteLast => {
                let size = self.counters.len();
                if size != 0 {
                    self.counters.guard().remove(size - 1);
                }
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
