use std::marker::PhantomData;

use gtk::{
    prelude::{BoxExt, ButtonExt, GtkWindowExt},
    traits::OrientableExt,
};
use relm4::gtk;
use relm4::{ComponentSender, RelmWidgetExt};
use relm4::factory::DynamicIndex;

#[derive(Default)]
struct Counter<T> {
    value: isize,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
enum CounterMsg {
    Change(isize),
}

#[derive(Debug)]
enum CounterOutput {
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
}

#[relm4::factory]
impl<T> relm4::factory::FactoryComponent for Counter<T>
where
    T: From<CounterOutput> + 'static + std::fmt::Debug,
{
    type Init = isize;
    type Input = CounterMsg;
    type Output = CounterOutput;
    type CommandOutput = ();
    type Widgets = CounterWidgets;
    type ParentInput = T;
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::Button {
                set_label: "-",
                connect_clicked[sender] => move |_| {
                    sender.input(CounterMsg::Change(-1))
                },
            },
            gtk::Label {
                #[watch]
                set_label: &format!("{}", self.value),
            },
            gtk::Button {
                set_label: "+",
                connect_clicked[sender] => move |_| {
                    sender.input(CounterMsg::Change(1))
                },
            },

            gtk::Button {
                set_label: "Up",
                connect_clicked[sender, index] => move |_| {
                    sender.output(CounterOutput::MoveUp(index.clone()))
                },
            },
            gtk::Button {
                set_label: "Down",
                connect_clicked[sender, index] => move |_| {
                    sender.output(CounterOutput::MoveDown(index.clone()))
                },
            },
        }
    }

    fn init_model(
        value: Self::Init,
        _index: &DynamicIndex,
        _sender: relm4::factory::FactorySender<Self>,
    ) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }

    fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output.into())
    }
}

impl<T> relm4::Component for Counter<T>
where
    T: From<CounterOutput> + 'static + std::fmt::Debug,
{
    type Init = isize;
    type Input = CounterMsg;
    type Output = CounterOutput;
    type CommandOutput = ();
    type Root = <Self as relm4::factory::FactoryComponent>::Root;
    type Widgets = CounterWidgets;

    fn init_root() -> Self::Root {
        <Self as relm4::factory::FactoryComponent>::init_root();
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        todo!()
    }
}

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

                Counter {

                },

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
