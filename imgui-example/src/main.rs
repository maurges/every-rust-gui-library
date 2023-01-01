mod support;

mod todo_item;

fn main() {
    let system = support::init(file!());
    let mut value = 0;

    let mut state = todo_item::TodoItem::new("foo".to_owned());
    let mut state2 = todo_item::TodoItem::new("foo".to_owned());

    system.main_loop(move |_, ui| {
        ui.window("Hello world")
            .size([300.0, 110.0], imgui::Condition::FirstUseEver)
            .build(|| {
                if ui.button("-") { value -= 1; }
                ui.same_line();
                ui.text(format!("{}", value));
                ui.same_line();
                if ui.button("+") { value += 1; }

                state.draw(ui);
                state2.draw(ui);

                if ui.button("print") {
                    eprintln!("{:?}", state.state);
                    eprintln!("{:?}", state2.state);
                }
            });
    });
}
