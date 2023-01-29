slint::slint!{
    import {Button} from "std-widgets.slint";
    export component HelloWorld inherits Window {
        VerticalLayout {
            text := Text {
                text: "hello world";
            }
            Button {
                text: "debug";
                width: 100px;
                height: 50px;

                clicked => {
                    text.color = #ff0000;
                }
            }
        }
    }
}

fn main() {
    HelloWorld::new().run();
}
