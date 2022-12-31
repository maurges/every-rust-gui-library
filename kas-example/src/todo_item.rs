use kas::prelude::{Widget, impl_scope, SetAccel, EventMgr, HasBool, HasStr, HasString};
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

trait AnyTextClass: kas::Widget + HasStr + HasString + 'static {}
impl<T> AnyTextClass for T where T: kas::Widget + HasStr + HasString + 'static {}
type AnyText = Box<dyn AnyTextClass>;

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
        text: widgets::Stack<AnyText>,
        #[widget]
        edit_button: widgets::TextButton,

        editing: bool,
    }
    impl Widget for Self {
        fn handle_message(&mut self, mgr: &mut EventMgr, _: usize) {
            if let Some(EditPressed) = mgr.try_pop_msg() {
                self.editing = !self.editing;
                let (accel, active, prev) = if self.editing {
                    ("done", 1, 0)
                } else {
                    ("edit", 0, 1)
                };
                *mgr |= self.edit_button.set_accel(accel);
                // set string of edit field
                let s = self.text[prev].get_string(); // fucking double borrow?
                *mgr |= self.text[active].set_string(s);
                mgr.config_mgr(|cgr| self.text.set_active(cgr, active));
            }
        }
    }
}

impl TodoItem {
    pub fn new(text: String) -> Self {
        let checkbox = widgets::CheckBox::new_on(|m, b| m.push_msg(CheckToggle(b)));
        let edit_button = widgets::TextButton::new_msg("edit", EditPressed);

        let text_show: AnyText = Box::new(widgets::StringLabel::new(text.clone()));
        let text_edit = Box::new(widgets::EditBox::new(text));
        let text = widgets::Stack::new_vec(vec![text_show, text_edit]);

        Self {
            core: Default::default(),
            checkbox,
            text,
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
            text: self.text.get_active().unwrap().get_string(),
            done: self.checkbox.get_bool(),
        }
    }
}
