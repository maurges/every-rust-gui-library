use iced::Sandbox;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    TodoItem::run(iced::Settings::default())?;
    Ok(())
}

struct TodoItem {
    text: String,
    is_done: bool,
    is_editing: bool,
}

#[derive(Debug, Clone)]
enum Message {
    DoneChanged(bool),
    EditingChanged,
    TextChanged(String),
}

impl Sandbox for TodoItem {
    type Message = Message;

    fn new() -> Self {
        TodoItem {
            text: "".to_owned(),
            is_done: false,
            is_editing: false,
        }
    }

    fn title(&self) -> String {
        "todo test".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::DoneChanged(x) => self.is_done = x,
            Message::EditingChanged => self.is_editing = !self.is_editing,
            Message::TextChanged(x) => self.text = x,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::row![
            iced::widget::checkbox("", self.is_done, Message::DoneChanged),
            if self.is_editing {
                iced::Element::from(
                    iced::widget::text_input("", &self.text, Message::TextChanged)
                        .width(200.into())
                )
            } else {
                iced::widget::text(self.text.clone())
                    .width(200.into())
                    .into()
            },
            iced::widget::button(if self.is_editing {"done"} else {"edit"})
                .on_press(Message::EditingChanged)
        ]
        .padding(20)
        .into()
    }
}
