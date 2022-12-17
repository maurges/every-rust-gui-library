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
    Save,
    Load,
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
            Message::Save => save(&self.items),
            Message::Load => match load() {
                Some(items) => self.items = items,
                None => (),
            }
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

fn save(items: &[TodoItem]) {
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

fn load() -> Option<Vec<TodoItem>> {
    let path = rfd::FileDialog::new().pick_file()?;
    match std::fs::File::open(path) {
        Err(e) => { eprintln!("{}", e); None }
        Ok(file) => match ron::de::from_reader(file) {
            Ok(items) => Some(items),
            Err(e) => { eprintln!("{}", e); None }
        }
    }
}
