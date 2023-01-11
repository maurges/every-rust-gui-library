use rui::{Modifiers, Binding};

use crate::lens;

#[derive(Debug, Default)]
pub struct TodoState {
    pub text: String,
    pub done: bool,
}

lens::make_bind!(crate::todo_item::TodoState, String, text);
lens::make_bind!(crate::todo_item::TodoState, bool, done);

pub fn todo_item(_cx: &rui::Context, state: impl rui::Binding<TodoState>) -> impl rui::View {
    use text::Bind as text_Bind;
    use done::Bind as done_Bind;
    rui::state(
        || false,
        move |is_editing, cx| rui::hstack((
            rui::toggle(state.done()).padding(rui::Auto),

            rui::cond(
                cx[is_editing],
                rui::text_editor(state.text()),
                state.text().get(cx).clone(),
            ),

            rui::button(
                if cx[is_editing] { "Done" } else { "Edit" },
                move |cx| cx[is_editing] = !cx[is_editing],
            ),
        ))
    )
}
