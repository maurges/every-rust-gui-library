mod todo_item;

use eframe::egui;
use todo_item::{TodoState, TodoItem};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    items: Vec<TodoItem>,
    item_add_text: String,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            items: Vec::new(),
            item_add_text: String::new(),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.item_add_text);
                if ui.button("Add").clicked() {
                    self.items.push(TodoItem::new(self.item_add_text.clone()));
                }
            });

            for item in &mut self.items {
                item.draw(ui);
            }

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    save(&self.items.iter().map(|x| x.state.clone()).collect::<Vec<_>>());
                }
                if ui.button("Load").clicked() {
                    if let Some(items) = load() {
                        self.items = items.into_iter().map(TodoItem::from_state).collect();
                    }
                }
            })
        });
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
