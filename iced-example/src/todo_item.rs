#[derive(serde::Serialize, serde::Deserialize)]
pub struct TodoState {
    pub text: String,
    pub done: bool,
}

#[derive(Clone)]
pub struct TodoItem {
    text: String,
    is_done: bool,
    is_editing: bool,
    index: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    DoneChanged(bool),
    EditingChanged,
    TextChanged(String),
}

pub struct TextChanged(pub String, pub usize);

impl TodoItem {
    pub fn new(text: String, index: usize) -> Self {
        TodoItem {
            text,
            is_done: false,
            is_editing: false,
            index,
        }
    }

    pub fn from_state(s: &TodoState, index: usize) -> Self {
        TodoItem {
            text: s.text.clone(),
            is_done: s.done,
            is_editing: false,
            index,
        }
    }
}

impl TodoState {
    pub fn new(text: String) -> Self {
        TodoState {
            text,
            done: false,
        }
    }
}

impl<Msg: From<TextChanged>> iced_lazy::Component<Msg, iced::Renderer> for TodoItem {
    type Event = Message;
    type State = ();

    fn update(&mut self, _: &mut (), message: Self::Event) -> Option<Msg> {
        match message {
            Message::DoneChanged(x) => self.is_done = x,
            Message::EditingChanged => if self.is_editing {
                self.is_editing = false;
                return Some(TextChanged(self.text.clone(), self.index).into());
            } else {
                self.is_editing = true;
            }
            Message::TextChanged(x) => self.text = x,
        }
        None
    }

    fn view(&self, _: &()) -> iced::Element<'_, Self::Event> {
        iced::widget::row![
            iced::widget::checkbox("", self.is_done, Message::DoneChanged),
            if self.is_editing {
                iced::Element::from(
                    iced::widget::text_input("", &self.text, Message::TextChanged)
                        .width(200.into()),
                )
            } else {
                iced::widget::text(self.text.clone())
                    .width(200.into())
                    .into()
            },
            iced::widget::button(if self.is_editing { "done" } else { "edit" })
                .on_press(Message::EditingChanged)
        ]
        .padding(20)
        .into()
    }
}

impl<'a, Message> From<TodoItem> for iced::Element<'a, Message, iced::Renderer>
where
    Message: 'a,
    Message: From<TextChanged>,
{
    fn from(numeric_input: TodoItem) -> Self {
        iced_lazy::component(numeric_input)
    }
}

impl From<TodoItem> for TodoState {
    fn from(x: TodoItem) -> Self {
        TodoState {
            text: x.text,
            done: x.is_done,
        }
    }
}
