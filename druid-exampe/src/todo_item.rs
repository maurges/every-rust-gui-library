use druid::{widget, Data, Lens, LensExt, Widget, WidgetExt};

#[derive(Clone, Data, Lens)]
pub struct TodoItem {
    pub text: String,
    pub done: bool,
}

#[derive(Clone, Data, Lens)]
pub struct TodoItemState {
    pub item_state: TodoItem,
    is_editing: bool,
}

impl Default for TodoItem {
    fn default() -> Self {
        Self {
            text: "".into(),
            done: false,
        }
    }
}

impl Default for TodoItemState {
    fn default() -> Self {
        Self {
            item_state: Default::default(),
            is_editing: false,
        }
    }
}

impl TodoItemState {
    pub fn new(item_state: TodoItem) -> Self {
        Self {
            item_state,
            is_editing: false,
        }
    }
}

pub fn todo_item() -> impl Widget<TodoItemState> {
    let button_label = widget::Label::dynamic(|data: &TodoItemState, _env| {
        if data.is_editing {
            "done".into()
        } else {
            "edit".into()
        }
    });
    let edit_button = widget::Button::from_label(button_label).on_click(
        |_ctx, state: &mut TodoItemState, _env| {
            state.is_editing = !state.is_editing;
        },
    );

    let text_box = widget::Either::new(
        |state, _env| state.is_editing,
        widget::TextBox::new().lens(TodoItemState::item_state.then(TodoItem::text)),
        widget::Label::dynamic(|data: &TodoItemState, _env| data.item_state.text.clone()),
    );

    widget::Flex::row()
        .with_child(
            widget::Checkbox::new("") // can I have a lack of text plz?
                .lens(TodoItemState::item_state.then(TodoItem::done)),
        )
        .with_child(text_box)
        .with_child(edit_button)
}
