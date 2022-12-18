// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Counter example (simple button)

use kas::prelude::{EventMgr, HasString, Widget, Window, impl_scope, SetAccel, HasStr};
use kas::widgets::{Label, TextButton};

#[derive(Clone, Debug)]
struct Increment(i32);

impl_scope! {
    #[widget{
        layout = column: [
            align(center): self.display,
            row: [ self.button_minus, self.button_plus ],
        ];
    }]
    #[derive(Debug)]
    struct Counter {
        core: widget_core!(),
        #[widget]
        display: Label<String>,
        #[widget]
        button_minus: TextButton,
        #[widget]
        button_plus: TextButton,
        count: i32,
    }
    impl Widget for Self {
        fn handle_message(&mut self, mgr: &mut EventMgr, _: usize) {
            if let Some(Increment(incr)) = mgr.try_pop_msg() {
                self.count += incr;
                *mgr |= self.display.set_string(self.count.to_string());
            }
            let current_text = self.button_plus.get_str();
            if current_text == "+" {
                *mgr |= self.button_plus.set_accel("++");
            } else {
                *mgr |= self.button_plus.set_accel("+");
            }
        }
    }
}

impl Counter {
    fn new(count: i32) -> Self {
        Counter {
            core: Default::default(),
            display: Label::from(count.to_string()),
            button_minus: TextButton::new_msg("−", Increment(-1)),
            button_plus: TextButton::new_msg("+", Increment(1)),
            count,
        }
    }
}
impl Window for Counter {
    fn title(&self) -> &str {
        "Counter"
    }
}

fn main() -> kas::shell::Result<()> {
    let theme = kas::theme::SimpleTheme::new().with_font_size(24.0);
    kas::shell::Toolkit::new(theme)?
        .with(Counter::new(0))?
        .run()
}
