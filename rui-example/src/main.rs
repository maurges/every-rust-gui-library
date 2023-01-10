use rui::{Auto, Modifiers, Lens};

#[derive(Default)]
struct CounterState(isize);

fn counter(cx: &rui::Context, count: impl rui::Binding<CounterState>) -> impl rui::View {
    rui::hstack((
        rui::button("decrement", move |cx| {
            count.get_mut(cx).0 -= 1;
        })
        .padding(Auto),

        count.get(cx).0,

        rui::button("increment", move |cx| {
            count.get_mut(cx).0 += 1;
        })
        .padding(Auto),
    ))
}

#[derive(Default)]
struct AppState {
    _1: CounterState,
    _2: CounterState,
    text_input: String,
}

rui::make_lens!(FstLens, AppState, CounterState, _1);
rui::make_lens!(SndLens, AppState, CounterState, _2);
rui::make_lens!(TextLens, AppState, String, text_input);

fn main() {
    rui::rui(rui::state(
        || Default::default(),
        |app_state, cx| {
            rui::vstack((
                counter(cx, rui::bind(app_state, FstLens {})),
                counter(cx, rui::bind(app_state, SndLens {})),
                counter(cx, rui::bind(app_state, FstLens {})),

                rui::text_editor(rui::bind(app_state, TextLens {})),
                rui::button("print", move |cx| {
                    let text = &cx[app_state].text_input;
                    eprintln!("{}", text);
                })
            ))
        },
    ));
}
