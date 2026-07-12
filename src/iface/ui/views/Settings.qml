import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    id: root

    RowLayout {
        anchors.fill: parent
        spacing: 0

        // navbar
        Rectangle {
            Layout.preferredWidth: parent.width * 0.2
            Layout.fillHeight: true
            color: "#f2f2f4"

            // list of settings groups
            ListView {
                anchors.fill: parent
                anchors.topMargin: 15
                spacing: 10

                model: ["Option 1", "Option 2", "Option 3", "Option 4"]

                delegate: Column {
                    width: ListView.view.width
                    spacing: 10

                    ItemDelegate {
                        id: delegate

                        width: parent.width

                        hoverEnabled: true

                        background: Rectangle {
                            color: delegate.hovered ? "#e6e6e6" : "transparent"
                        }

                        contentItem: Label {
                            text: modelData
                            anchors.fill: parent
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                    }

                    Rectangle {
                        visible: index === 1
                        width: parent.width * 0.9
                        height: 1
                        color: "#c0c0c0"
                        anchors.horizontalCenter: parent.horizontalCenter
                    }
                }
            }
        }

        // content area
        Rectangle {
            Layout.fillHeight: true
            Layout.fillWidth: true
            color: "white"

            Label {
                anchors.centerIn: parent
                text: "Settings Content"
            }
        }
    }
}