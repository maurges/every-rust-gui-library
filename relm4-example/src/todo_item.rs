use std::marker::PhantomData;

use relm4::{gtk, RelmWidgetExt};
use gtk::traits::{OrientableExt, BoxExt};
use gtk::prelude::{ CheckButtonExt, ButtonExt, EntryBufferExtManual, WidgetExt };

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TodoState {
    text: String,
    done: bool,
}

impl TodoState {
    pub fn new(text: String) -> Self {
        Self {
            text,
            done: false,
        }
    }
}

#[derive(Default)]
pub struct TodoItem<T> {
    input_buffer: gtk::EntryBuffer,
    editing: bool,
    pub state: TodoState,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
pub enum Input {
    Checked,
    EditPressed,
}

#[relm4::factory(pub)]
impl<T> relm4::factory::FactoryComponent for TodoItem<T>
where
    T: 'static + std::fmt::Debug
{
    type Input = Input;
    type Output = ();
    type Init = TodoState;
    type CommandOutput = ();
    type Widgets = TodoItemWidgets;
    type ParentInput = T;
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::CheckButton {
                connect_toggled => Input::Checked,
            },
            gtk::Label {
                #[watch]
                set_label: &self.state.text,
                // set_width: 200,
                #[watch]
                set_visible: !self.editing,
            },
            gtk::Entry::with_buffer(&self.input_buffer) {
                #[watch]
                set_visible: self.editing,
            },
            gtk::Button {
                #[watch]
                set_label: if self.editing { "done" } else { "edit" },
                connect_clicked => Input::EditPressed,
            }
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: relm4::factory::FactorySender<Self>) {
        match msg {
            Input::Checked => self.state.done = !self.state.done,
            Input::EditPressed => {
                self.editing = !self.editing;
                if self.editing {
                    self.input_buffer.set_text(&self.state.text);
                } else {
                    self.state.text = self.input_buffer.text();
                }
            }
        }
    }

    fn init_model(
        state: Self::Init,
        _index: &relm4::prelude::DynamicIndex,
        _sender: relm4::factory::FactorySender<Self>,
    ) -> Self {
        Self {
            state,
            input_buffer: gtk::EntryBuffer::new(None),
            editing: false,
            phantom: PhantomData,
        }
    }

    fn output_to_parent_input((): ()) -> Option<Self::ParentInput> {
        None
    }
}
