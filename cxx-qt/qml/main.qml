import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12
import QtQuick.Layouts 1.12

import com.kdab.cxx_qt.demo 1.0

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

                    placeholderText: qstr("New item")
                }

                Button {
                    text: qsTr("Add")
                }
            }

            TodoItem {
                text: "todo"
                Layout.fillWidth: true
                Layout.fillHeight: false
            }

            Item {
                // spacer
                Layout.fillHeight: true
                Layout.fillWidth: true
            }

            Row {
                Button {
                    text: qsTr("save")
                }
                Button {
                    text: qsTr("load")
                }
            }
        }
    }
}
