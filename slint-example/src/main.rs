slint::include_modules!();

fn main() {
    let window = UiMain::new();
    let globals = window.global::<Globals>();
    globals.on_make_model(|ModelImpurity { foo }| {
        let model = slint::VecModel::from_slice(&[
            OneInt { value: 10 },
            OneInt { value: 15 },
            OneInt { value: foo },
        ]);
        slint::ModelRc::new(model)
    });
    globals.on_initialize_model(|| {
        eprintln!("impurity");
        ModelImpurity {
            foo: -1,
        }
    });
    window.run()
}
