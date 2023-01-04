import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

RowLayout {
    id: todoItem

    property alias checked: checkbox.checked
    property string text

    Layout.fillHeight: false

    CheckBox {
        id: checkbox
        Layout.fillHeight: true
    }

    Item {
        Layout.fillHeight: true
        Layout.fillWidth: true

        Label {
            id: label
            anchors.fill: parent
            text: todoItem.text
        }

        TextField {
            id: input
            anchors.fill: parent
            visible: false

            text: todoItem.text
        }
    }

    Button {
        id: editButton
        Layout.fillHeight: true
        text: qsTr("edit")

        onClicked: {
            if (todoItem.state == "editing") {
                todoItem.text = input.text;
                todoItem.state = "";
            } else {
                todoItem.state = "editing";
                input.focus = true;
            }
        }
    }

    states: [
        State {
            name: "editing"
            PropertyChanges {
                target: editButton
                text: qsTr("done")
            }
            PropertyChanges {
                target: label
                visible: false
            }
            PropertyChanges {
                target: input
                visible: true
            }
        }
    ]
}
