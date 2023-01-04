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
                    text: modelText
                    checked: modelDone

                    onTextChanged: {
                        console.log("text changed: %1".arg(text));
                        modelText = text;
                        console.log("in model: %1".arg(modelText));
                    }
                    onCheckedChanged: modelDone = checked
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
                    onClicked: {
                        console.log("saving...");
                        for (let i = 0; i < listModel.len(); ++i) {
                            let x = listModel.get(i);
                            console.log("got: %1 %2".arg(x.modelText).arg(x.modelDone));
                            rust.addItem(x.modelText, x.modelDone);
                        }
                        saveDialog.open();
                    }
                }
                Button {
                    text: qsTr("load")
                    onClicked: {
                        loadDialog.open();
                    }
                }
            }
        }
    }

    FileDialog {
        id: saveDialog
        selectMultiple: false
        selectExisting: false
        onAccepted: {
            let r = rust.saveItems(saveDialog.fileUrl);
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
            let r = rust.loadItems(loadDialog.fileUrl);
            if (r !== "") {
                console.error("couldn't load: %1".arg(r));
            } else {
                listModel.clear();
                while (rust.nextItem()) {
                    let text = rust.nextItemText();
                    let checked = rust.nextItemDone();
                    let item = listModel.make(text);
                    item.done = checked;
                    listModel.append(item);
                }
            }
        }
    }

    TodoListModel {
        id: listModel
    }

    MyObject {
        id: rust
    }
}
