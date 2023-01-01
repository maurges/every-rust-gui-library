use eframe::egui::Ui;

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

    pub fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.state.done, "");

            if self.editing {
                ui.text_edit_singleline(&mut self.state.text);
                if ui.button("done").clicked() {
                    self.editing = false;
                }
            } else {
                ui.label(&self.state.text);
                if ui.button("edit").clicked() {
                    self.editing = true;
                }
            }
        });
    }
}
