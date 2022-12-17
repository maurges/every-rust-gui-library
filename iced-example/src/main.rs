mod todo_item;

use iced::Sandbox;

use todo_item::{TodoItem, TodoState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    TodoApp::run(iced::Settings::default())?;
    Ok(())
}

struct TodoApp {
    create_text: String,
    items: Vec<TodoState>,
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
    ItemTextChanged(String, usize),
    CreateNew,
    Save,
    Load,
}

impl From<todo_item::TextChanged> for Message {
    fn from(todo_item::TextChanged(x, i): todo_item::TextChanged) -> Self {
        Message::ItemTextChanged(x, i)
    }
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
            Message::CreateNew => self.items.push(TodoState::new(self.create_text.clone())),
            Message::Save => save(&self.items),
            Message::Load => match load() {
                Some(items) => self.items = items,
                None => (),
            }
            Message::ItemTextChanged(text, index) => self.items[index].text = text,
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

        content.extend(self.items.iter().enumerate().map(|(i, x)| TodoItem::from_state(x, i).into()));

        let save_load = iced::widget::row![
            iced::widget::button("Save")
                .on_press(Message::Save),
            iced::widget::button("Load")
                .on_press(Message::Load),
        ];
        content.push(save_load.into());

        iced::widget::column(content)
            .padding(20)
            .align_items(iced::Alignment::Center)
            .into()
    }
}

fn save(items: &[TodoState]) {
    if let Some(path) = rfd::FileDialog::new().save_file() {
        match std::fs::File::create(&path) {
            Ok(file) => match ron::ser::to_writer(file, items) {
                Ok(()) => (),
                Err(e) => eprintln!("{}", e),
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn load() -> Option<Vec<TodoState>> {
    let path = rfd::FileDialog::new().pick_file()?;
    match std::fs::File::open(path) {
        Err(e) => { eprintln!("{}", e); None }
        Ok(file) => match ron::de::from_reader(file) {
            Ok(items) => Some(items),
            Err(e) => { eprintln!("{}", e); None }
        }
    }
}
