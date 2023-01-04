import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12
import QtQuick.Layouts 1.12
import QtQuick.Dialogs 1.3

import men.morj.qmetaobject 1.0

Window {
    id: window
    height: 480
    title: qsTr("todo list")
    visible: true
    width: 640

    ScrollView {
        anchors.fill: parent

        ColumnLayout {
            x: 10
            y: 10
            height: Math.max(implicitHeight, window.height)
            width: window.width - 20
            RowLayout {
                TextField {
                    id: input
                    Layout.fillWidth: true

                    placeholderText: qsTr("New item")
                }

                Button {
                    text: qsTr("Add")

                    onClicked: {
                        listModel.append(listModel.make(input.text));
                        input.text = "";
                    }
                }
            }

            Repeater {
                model: listModel
                delegate: TodoItem {
                    text: model.text
                    checked: model.done

                    onTextChanged: {
                        model.text = text;
                    }
                    onCheckedChanged: model.done = checked
                }
            }

            Item {
                // spacer
                Layout.fillHeight: true
                Layout.fillWidth: true
            }

            Row {
                Button {
                    text: qsTr("save")
                    onClicked: { saveDialog.open(); }
                }
                Button {
                    text: qsTr("load")
                    onClicked: { loadDialog.open(); }
                }
            }
        }
    }

    FileDialog {
        id: saveDialog
        selectMultiple: false
        selectExisting: false
        onAccepted: {
            let r = listModel.save_items(saveDialog.fileUrl);
            if (r !== "") {
                console.error("couldn't save: %1".arg(r));
            }
        }
    }
    FileDialog {
        id: loadDialog
        selectMultiple: false
        selectExisting: true
        onAccepted: {
            let r = listModel.load_items(loadDialog.fileUrl);
            if (r !== "") {
                console.error("couldn't load: %1".arg(r));
            }
        }
    }

    TodoListModel {
        id: listModel
    }
}
