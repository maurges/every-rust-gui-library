use std::marker::PhantomData;

use gtk::{
    prelude::{BoxExt, ButtonExt},
    traits::OrientableExt,
};
use relm4::gtk;
use relm4::RelmWidgetExt;
use relm4::factory::DynamicIndex;

#[derive(Default)]
pub struct Counter<T> {
    value: isize,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
pub enum CounterMsg {
    Change(isize),
}

#[derive(Debug)]
pub enum CounterOutput {
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
    Delete(DynamicIndex),
}

#[relm4::factory(pub)]
impl<T> relm4::factory::FactoryComponent for Counter<T>
where
    T: From<CounterOutput> + 'static + std::fmt::Debug,
{
    type Init = isize;
    type Input = CounterMsg;
    type Output = CounterOutput;
    type CommandOutput = ();
    type Widgets = CounterFactoryWidgets;
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
            gtk::Button {
                set_label: "Del",
                connect_clicked[sender, index] => move |_| {
                    sender.output(CounterOutput::Delete(index.clone()))
                },
            },
        }
    }

    fn update(&mut self, CounterMsg::Change(delta): Self::Input, _sender: relm4::factory::FactorySender<Self>) {
        self.value += delta;
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
/*
#[relm4::component]
impl relm4::SimpleComponent for Counter<()>
{
    type Input = CounterMsg;
    type Output = ();
    type Init = isize;
    type Widgets = CounterWidgets;

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

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let widgets = view_output!();
        todo!()
    }

    fn update(
        &mut self,
        CounterMsg::Change(delta): Self::Input,
        _sender: ComponentSender<Self>,
    ) {
        todo!()
    }
}
*/
