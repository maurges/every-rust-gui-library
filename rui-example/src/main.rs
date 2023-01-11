pub mod lens;
mod todo_item;

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

lens::make_bind!(crate::AppState, crate::CounterState, _1);
lens::make_bind!(crate::AppState, crate::CounterState, _2);
lens::make_bind!(crate::AppState, String, text_input);

fn main() {
    use _1::Bind as _1_Bind;
    use _2::Bind as _2_Bind;
    use text_input::Bind as text_input_Bind;
    rui::rui(rui::state(
        || Default::default(),
        |app_state, cx| {
            rui::vstack((
                counter(cx, app_state._1()),
                counter(cx, app_state._2()),
                counter(cx, app_state._1()),

                rui::state(
                    || todo_item::TodoState { text: "kekus".into(), done: true },
                    |local_state, cx| todo_item::todo_item(cx, local_state),
                ),

                rui::text_editor(app_state.text_input()),
                rui::button("print", move |cx| {
                    let text = &cx[app_state].text_input;
                    eprintln!("{}", text);
                }),
            ))
        },
    ));
}
