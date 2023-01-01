use imgui::Ui;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TodoState {
    pub text: String,
    pub done: bool,
}

pub struct TodoItem {
    pub state: TodoState,
    editing: bool,
}

impl TodoItem {
    pub fn new(text: String) -> Self {
        Self {
            state: TodoState {
                text,
                done: false,
            },
            editing: false,
        }
    }

    pub fn from_state(state: TodoState) -> Self {
        Self {
            state,
            editing: false,
        }
    }

    pub fn draw(&mut self, ui: &Ui) {
        ui.checkbox(" ", &mut self.state.done);
        ui.same_line();

        if self.editing {
            ui.input_text("  ", &mut self.state.text).build();
            ui.same_line();
            if ui.button("done") {
                self.editing = false;
            }
        } else {
            ui.text(&self.state.text);
            ui.same_line();
            if ui.button("edit") {
                self.editing = true;
            }
        }
    }
}
