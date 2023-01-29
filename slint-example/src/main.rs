slint::include_modules!();

fn main() {
    let window = UiMain::new();
    window.global::<Globals>().on_make_model(|| {
        let model = slint::VecModel::from_slice(&[
            OneInt { value: 10 },
            OneInt { value: 15 },
            OneInt { value: -1 },
        ]);
        slint::ModelRc::new(model)
    });
    window.run()
}
