use ribir::prelude::*;

fn main() {
  let counter = fn_widget! {
    let row = @Row{};
    let count1 = State::value(0);
    let count2 = State::value(0);
    let total = pipe!(*$count1 + *$count2);
    @$row {
      @FilledButton {
        on_tap: move |_| *$count1.write() += 1,
        @{ Label::new("Increment") }
      }
      @H1{ text: total.map(|x| x.to_string()) }
      @FilledButton {
        on_tap: move |_| *$count2.write() -= 1,
        @{ Label::new("Decrement") }
      }
    }
  };
  App::run(counter);
}
