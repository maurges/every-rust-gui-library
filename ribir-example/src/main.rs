use ribir::prelude::*;

struct Counter(usize);

impl Compose for Counter {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @Row {
        @FilledButton {
          on_tap: move |_| $this.write().0 += 1,
          @{Label::new("Increment")}
        }
        @H1 {
          text: pipe!((*$this).0.to_string()),
          margin: EdgeInsets::all(10.0),
        }
      }
    }
  }
}

fn main() {
  let counter = fn_widget! {
    Counter(0)
  };
  App::run(counter);
}
