use relm4::gtk;
use gtk::{prelude::{BoxExt, ButtonExt, GtkWindowExt}, glib::clone};
use relm4::RelmWidgetExt;

#[derive(Default)]
struct AppModel {
    counter: isize,
}

#[derive(Debug)]
enum AppMsg {
    Change(isize),
}

struct AppWidgets {
    label: gtk::Label,
}

impl relm4::SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = isize;
    type Root = gtk::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("hello gtk")
            .default_width(640)
            .default_height(480)
            .build()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { counter: init };

        let hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();

        let inc_button = gtk::Button::with_label("Increment");
        let dec_button = gtk::Button::with_label("Decrement");

        let label = gtk::Label::new(Some(&format!("Counter: {}", model.counter)));
        label.set_margin_all(5);

        root.set_child(Some(&hbox));
        hbox.set_margin_all(5);
        hbox.append(&dec_button);
        hbox.append(&label);
        hbox.append(&inc_button);

        inc_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppMsg::Change(1));
        }));

        dec_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppMsg::Change(-1));
        }));

        let widgets = AppWidgets { label };

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, AppMsg::Change(delta): Self::Input, _sender: relm4::ComponentSender<Self>) {
        self.counter += delta;
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: relm4::ComponentSender<Self>) {
        widgets.label.set_label(&format!("Counter: {}", self.counter));
    }
}

fn main() {
    let app = relm4::RelmApp::new("dafuq.is.this");
    app.run::<AppModel>(0);
}
