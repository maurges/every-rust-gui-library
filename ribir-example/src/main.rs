use ribir::prelude::*;

struct Counter {
  state: State<usize>,
}

impl Counter {
  pub fn new() -> Self {
    Self {
      state: State::value(0),
    }
  }
}

impl Compose for Counter {
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

impl StateReader for Counter {
  type Value = usize;
  type OriginReader = <State<usize> as StateReader>::OriginReader;
  type Reader = <State<usize> as StateReader>::Reader;

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

impl StateWriter for Counter {
  type Writer = <State<usize> as StateWriter>::Writer;
  type OriginWriter = <State<usize> as StateWriter>::OriginWriter;

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
    let counter = @{ Counter::new() };
    let input = @Input {};
    let button = @FilledButton {
      on_tap: move |_| { let x = *$counter; eprintln!("counter: {}", x)},
      @{Label::new("get")}
    };
    @Column {
      @{ counter }
      @$input {}
      @$button {}
    }
  };
  App::run(app);
}
