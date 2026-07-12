import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    id: root

    property real preferredWidth: 280

    signal backClicked()

    Layout.preferredWidth: preferredWidth
    Layout.fillHeight: true

    Rectangle {
        anchors.fill: parent
        
        color: "#f2f2f4"

        Column {
            anchors {
                top: parent.top
                horizontalCenter: parent.horizontalCenter
                topMargin: 5
            }

            width: parent.width * 0.93

            Item {
                width: parent.width
                height: 45

                // Back button
                SimpleButton {
                    anchors.left: parent.left
                    anchors.verticalCenter: parent.verticalCenter

                    height: parent.height * 0.8
                    width: height
                    
                    text: "h"

                    onClicked: root.backClicked()
                }

                // Page title
                Label {
                    anchors.centerIn: parent

                    text: "Settings"

                    font {
                        pixelSize: 15
                        weight: Font.DemiBold
                    }
                }
            }
            

            // List of settings groups
            ListView {
                spacing: 10

                width: parent.width
                height: 300

                model: ["Option 1", "Option 2", "Option 3", "Option 4"]

                delegate: ItemDelegate {
                    width: ListView.view.width
                    height: 45

                    hoverEnabled: true

                    background: Rectangle {
                        color: hovered ? "#e6e6e6" : "transparent"
                        radius: 6
                    }

                    contentItem: Label {
                        text: modelData
                        anchors.fill: parent
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }
            }
        }
    }
}