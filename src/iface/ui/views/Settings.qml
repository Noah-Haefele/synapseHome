import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Effects

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
            // back button
            Button {
                id: backButton

                anchors {
                    left: parent.left
                    right: parent.right
                    top: parent.top
                    margins: 5
                }

                height: 45
                
                background: Item {
                    Rectangle {
                        id: bg
                        anchors.fill: parent
                        radius: 10
                        color: backButton.down ? "#e8e8e8" : "#f2f2f4"
                    }
                }

                contentItem: Text {
                    text: "Home"
                    color: "#222"
                    font.pixelSize: 17
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
            }

            // list of settings groups
            ListView {
                anchors {
                    left: parent.left
                    right: parent.right
                    top: backButton.bottom
                    bottom: parent.bottom

                    leftMargin: 5
                    rightMargin: 5
                    topMargin: 15
                    bottomMargin: 5
                }

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
                        width: parent.width
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