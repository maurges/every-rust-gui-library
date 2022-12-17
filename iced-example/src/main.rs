mod todo_item;

use iced::Sandbox;

use todo_item::TodoItem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    TodoApp::run(iced::Settings::default())?;
    Ok(())
}

struct TodoApp {
    create_text: String,
    items: Vec<TodoItem>,
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
    CreateNew,
}

impl Sandbox for TodoApp {
    type Message = Message;

    fn new() -> Self {
        TodoApp {
            create_text: String::new(),
            items: Vec::new(),
        }
    }

    fn title(&self) -> String {
        "todo app".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextChanged(x) => self.create_text = x,
            Message::CreateNew => self.items.push(TodoItem::new(self.create_text.clone())),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut content = Vec::new();

        let creation = iced::widget::row![
            iced::widget::text_input("New item", &self.create_text, Message::TextChanged)
                .width(150.into()),
            iced::widget::button("Add")
                .on_press(Message::CreateNew)
        ];
        content.push(creation.into());
        content.extend(self.items.iter().map(|x| x.clone().into()));

        iced::widget::column(content)
            .padding(20)
            .align_items(iced::Alignment::Center)
            .into()
    }
}
