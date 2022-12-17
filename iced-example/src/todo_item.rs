#[derive(Clone)]
pub struct TodoItem {
    text: String,
    is_done: bool,
    is_editing: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    DoneChanged(bool),
    EditingChanged,
    TextChanged(String),
}

impl TodoItem {
    pub fn new(text: String) -> Self {
        TodoItem {
            text,
            is_done: false,
            is_editing: false,
        }
    }
}

impl<Msg> iced_lazy::Component<Msg, iced::Renderer> for TodoItem {
    type Event = Message;
    type State = ();

    fn update(&mut self, _: &mut (), message: Self::Event) -> Option<Msg> {
        match message {
            Message::DoneChanged(x) => self.is_done = x,
            Message::EditingChanged => self.is_editing = !self.is_editing,
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
{
    fn from(numeric_input: TodoItem) -> Self {
        iced_lazy::component(numeric_input)
    }
}
