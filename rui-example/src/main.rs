pub mod lens;

use rui::Modifiers;

#[derive(Default)]
struct CounterState(isize);

fn counter(cx: &rui::Context, count: impl rui::Binding<CounterState>) -> impl rui::View {
    rui::hstack((
        rui::button("decrement", move |cx| {
            count.get_mut(cx).0 -= 1;
        })
        .padding(rui::Auto),

        count.get(cx).0,

        rui::button("increment", move |cx| {
            count.get_mut(cx).0 += 1;
        })
        .padding(rui::Auto),
    ))
}

#[derive(Default)]
struct AppState {
    _1: CounterState,
    _2: CounterState,
    text_input: String,
}

lens::make_bind!(AppState, CounterState, _1);
lens::make_bind!(AppState, CounterState, _2);
lens::make_bind!(AppState, String, text_input);

fn main() {
    rui::rui(rui::state(
        || Default::default(),
        |app_state, cx| {
            rui::vstack((
                counter(cx, AppState::_1(app_state)),
                counter(cx, AppState::_2(app_state)),
                counter(cx, AppState::_1(app_state)),

                rui::text_editor(AppState::text_input(app_state)),
                rui::button("print", move |cx| {
                    let text = &cx[app_state].text_input;
                    eprintln!("{}", text);
                })
            ))
        },
    ));
}
