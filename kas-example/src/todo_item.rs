use kas::prelude::{Widget, impl_scope, SetAccel, EventMgr, HasBool, HasStr};
use kas::{widgets, TkAction};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TodoState {
    pub text: String,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct CheckToggle(pub bool);

#[derive(Clone, Debug)]
struct EditPressed;

impl_scope! {
    #[widget{
        layout = row: [
            self.checkbox,
            self.text,
            self.edit_button,
        ];
    }]
    #[derive(Debug)]
    pub struct TodoItem {
        core: widget_core!(),
        #[widget]
        checkbox: widgets::CheckBox,
        #[widget]
        text: widgets::StringLabel,
        #[widget]
        edit_button: widgets::TextButton,

        editing: bool,
    }
    impl Widget for Self {
        fn handle_message(&mut self, mgr: &mut EventMgr, _: usize) {
            if let Some(EditPressed) = mgr.try_pop_msg() {
                self.editing = !self.editing;
                *mgr |= self.edit_button.set_accel(
                    if self.editing { "done" } else { "edit" }
                );
            }
        }
    }
}

impl TodoItem {
    pub fn new(text: String) -> Self {
        let checkbox = widgets::CheckBox::new_on(|m, b| m.push_msg(CheckToggle(b)));
        let edit_button = widgets::TextButton::new_msg("edit", EditPressed);

        Self {
            core: Default::default(),
            checkbox,
            text: widgets::StringLabel::new(text),
            edit_button,

            editing: false,
        }
    }

    pub fn set_done(&mut self, done: bool) -> TkAction {
        self.checkbox.set_bool(done)
    }
    /* // might need this later, but probably not
    pub fn set_text(&mut self, text: String) -> TkAction {
        self.text.set_text(text)
    }
    */

    pub fn get_state(&self) -> TodoState {
        TodoState {
            text: self.text.get_string(),
            done: self.checkbox.get_bool(),
        }
    }
}
