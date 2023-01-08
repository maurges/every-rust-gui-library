use relm4::gtk;
use relm4::ComponentSender;
use relm4::gtk::traits::ButtonExt;
use relm4::gtk::traits::OrientableExt;
use relm4::gtk::traits::WidgetExt;

#[derive(Debug)]
pub enum Output {
    Add(isize),
    DeleteLast,
}

#[derive(Debug)]
pub enum Input {
    AddPressed,
    SpinnerChanged(f64),
}

#[derive(Default)]
pub struct Controls {
    spinner_value: isize,
}

#[relm4::component(pub)]
impl relm4::SimpleComponent for Controls {
    type Input = Input;
    type Output = Output;
    type Init = ();
    type Widgets = ControlsWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,

            #[name = "spinner"]
            gtk::SpinButton {
                set_numeric: true,
                set_width_request: 500,

                connect_value_changed[sender] => move |this| {
                    sender.input(Input::SpinnerChanged(this.value()))
                }
            },

            gtk::Button {
                set_label: "Add new",
                connect_clicked[sender] => move |_| {
                    sender.input(Input::AddPressed)
                }
            },

            gtk::Button {
                set_label: "Delete",
                connect_clicked[sender] => move |_| {
                    let _ = sender.output(Output::DeleteLast);
                },
            }
        }
    }

    fn init((): (), root: &Self::Root, sender: relm4::ComponentSender<Self>) -> relm4::ComponentParts<Self> {
        let model = Controls {
            spinner_value: 0,
        };
        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        msg: Self::Input,
        sender: relm4::ComponentSender<Self>,
    ) {
        eprintln!("message");
        match msg {
            Input::AddPressed => {
                let _ = sender.output(Output::Add(self.spinner_value));
            },
            Input::SpinnerChanged(val) => {
                eprintln!("spinner changed");
                self.spinner_value = val.floor() as isize;
            }
        }
    }
}
