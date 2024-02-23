use ribir::prelude::*;

struct Counter<S> {
  state: S,
}

impl Counter<State<usize>> {
  pub fn new() -> Self {
    Self {
      state: State::value(0),
    }
  }
}

impl<S> Counter<S> {
  pub fn new_from(state: S) -> Self {
    Self {state}
  }
}

impl Clone for Counter<State<usize>> {
  fn clone(&self) -> Self {
    Counter {
      state: State::value(self.state.get())
    }
  }
}

impl<S> Compose for Counter<S>
where
    S: StateWriter<Value = usize>,
{
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @Row {
        @FilledButton {
          on_tap: move |_| *($this.state.write()) += 1,
          @{Label::new("Increment")}
        }
        @H1 {
          text: {
            let inner = &$this.state;
            pipe!($inner.to_string())
          },
          margin: EdgeInsets::all(10.0),
        }
      }
    }
  }
}

impl<S: StateReader> StateReader for Counter<S> {
  type Value = S::Value;
  type OriginReader = S::OriginReader;
  type Reader = S::Reader;

  fn read(&self) -> ReadRef<Self::Value> {
    self.state.read()
  }

  fn clone_reader(&self) -> Self::Reader {
    self.state.clone_reader()
  }

  fn origin_reader(&self) -> &Self::OriginReader {
    self.state.origin_reader()
  }

  fn time_stamp(&self) -> std::time::Instant {
    self.state.time_stamp()
  }

  fn raw_modifies(&self) -> ops::box_it::BoxOp<'static, ModifyScope, std::convert::Infallible> {
    self.state.raw_modifies()
  }

  fn try_into_value(self) -> Result<Self::Value, Self>
    where
      Self: Sized,
      Self::Value: Sized
  {
    self.state.try_into_value().map_err(|e| Self { state: e })
  }
}

impl<S: StateWriter> StateWriter for Counter<S> {
  type Writer = S::Writer;
  type OriginWriter = S::OriginWriter;

  fn write(&self) -> WriteRef<Self::Value> {
    self.state.write()
  }

  fn silent(&self) -> WriteRef<Self::Value> {
    self.state.silent()
  }

  fn shallow(&self) -> WriteRef<Self::Value> {
    self.state.shallow()
  }

  fn clone_writer(&self) -> Self::Writer {
    self.state.clone_writer()
  }

  fn origin_writer(&self) -> &Self::OriginWriter {
    self.state.origin_writer()
  }
}

fn main() {
  let app = fn_widget! {
    let full_state = State::value((0usize, 0usize));
    let proj_l = full_state.map_writer(|t| &t.0, |t| &mut t.0);
    let proj_r = full_state.map_writer(|t| &t.1, |t| &mut t.1);
    let counter_l = Counter::new_from(proj_l);
    let counter_r = Counter::new_from(proj_r);
    let button_print_full = @FilledButton {
      on_tap: move |_| { let x = *$full_state; eprintln!("full value: {:?}", x) },
      @{Label::new("get two")}
    };

    let counter = Counter::new();
    let input = @Input {};
    let button = @FilledButton {
      on_tap: move |_| { let x = *$counter; eprintln!("counter: {}", x)},
      @{Label::new("get")}
    };
    @VScrollBar {
      @Column {
        @$counter_l {}
        @$counter_r {}
        @$button_print_full {}
        @{
          (0..10).map(|_| @{counter.clone()})
        }
        @$input {}
        @$button {}
        @Container {
          size: Size::new(400.0, 400.0),
          background: Color::RED,
        }
      }
    }
  };
  App::run(app);
}
