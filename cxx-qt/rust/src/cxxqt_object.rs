#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct TodoItem {
    text: String,
    done: bool,
}

struct MyObjectState {
    items: Vec<TodoItem>,
    reading_index: Option<usize>,
}

#[cxx_qt::bridge]
mod my_object {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }

    #[cxx_qt::qobject]
    pub struct MyObject {
        state: super::MyObjectState,
    }

    impl Default for MyObject {
        fn default() -> Self {
            Self {
                state: super::MyObjectState {
                    items: Vec::new(),
                    reading_index: None,
                }
            }
        }
    }

    impl qobject::MyObject {
        #[qinvokable]
        pub fn add_item(self: Pin<&mut Self>, text: &QString, done: bool) {
            let item = super::TodoItem {
                text: text.to_string(),
                done,
            };
            self.state_mut().items.push(item);
        }

        #[qinvokable]
        pub fn save_items(self: Pin<&mut Self>, path: &QUrl) -> QString {
            let dest = path.qstring().to_string();
            let dest = dest.strip_prefix("file://").unwrap_or(&dest);
            let r = match std::fs::File::create(dest) {
                Ok(file) => match ron::ser::to_writer(file, &self.state().items) {
                    Ok(()) => "".to_owned(),
                    Err(e) => format!("{}", e),
                }
                Err(e) => format!("{}", e),
            };
            self.state_mut().items = Vec::new();
            eprintln!("saved");
            QString::from(&r)
        }

        #[qinvokable]
        pub fn load_items(self: Pin<&mut Self>, path: &QUrl) -> QString {
            let src = path.qstring().to_string();
            eprintln!("loading {} = {}", path, src);
            let src = src.strip_prefix("file://").unwrap_or(&src);
            match std::fs::File::open(src) {
                Ok(file) => match ron::de::from_reader(file) {
                    Ok(items) => {
                        let mut state = self.state_mut();
                        state.items = items;
                        QString::from("")
                    }
                    Err(e) => QString::from(&format!("{}", e)),
                }
                Err(e) => QString::from(&format!("{}", e)),
            }
        }

        #[qinvokable]
        pub fn next_item(self: Pin<&mut Self>) -> bool {
            let mut state = self.state_mut();
            match state.reading_index {
                None if state.items.is_empty() => false,
                None => {
                    state.reading_index = Some(0);
                    true
                }
                Some(mut i) => {
                    i += 1;
                    state.reading_index = Some(i);
                    if i < state.items.len() {
                        true
                    } else {
                        state.items = Vec::new();
                        false
                    }
                }
            }
        }

        #[qinvokable]
        pub fn next_item_text(&self) -> QString {
            let state = self.state();
            match state.reading_index {
                None => "".into(),
                Some(i) => QString::from(&state.items[i].text),
            }
        }
        #[qinvokable]
        pub fn next_item_done(&self) -> bool {
            let state = self.state();
            match state.reading_index {
                None => false,
                Some(i) => state.items[i].done,
            }
        }
    }
}
