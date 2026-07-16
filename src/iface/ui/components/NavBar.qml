import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../models"

Item {
    id: root

    property real preferredWidth: 280

    property color backgroundColor: "#f2f2f4"

    signal backClicked()

    Layout.preferredWidth: preferredWidth
    Layout.fillHeight: true

    Rectangle {
        anchors.fill: parent
        
        color: root.backgroundColor

        ColumnLayout {
            anchors {
                top: parent.top
                bottom: parent.bottom
                horizontalCenter: parent.horizontalCenter
                topMargin: 5
            }

            width: parent.width * 0.93

            // Header
            Item {
                Layout.fillWidth: true
                Layout.preferredHeight: 45

                // Back button
                SimpleButton {
                    anchors.left: parent.left
                    anchors.verticalCenter: parent.verticalCenter

                    height: parent.height * 0.8
                    width: height

                    backgroundColor: root.backgroundColor
                    textColor: "black"
                    
                    text: "\u2190"

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

                // System action button
                SimpleButton {
                    anchors.right: parent.right
                    anchors.verticalCenter: parent.verticalCenter

                    height: parent.height * 0.8
                    width: height

                    backgroundColor: root.backgroundColor
                    textColor: "black"
                    
                    text: "\u23FB"
                }
            }

            // List of settings groups
            ListView {
                id: floorList

                Layout.fillWidth: true
                Layout.fillHeight: true

                spacing: 10

                topMargin: 10

                model: Floor {}

                // Disable user scrolling
                interactive: false

                delegate: Rectangle {
                    id: wrapper

                    width: ListView.view.width
                    height: 45

                    readonly property bool isCurrent: ListView.isCurrentItem

                    Rectangle {
                        id: background

                        anchors.fill: parent

                        radius: 6
                        color: wrapper.isCurrent ? "#79baff" : root.backgroundColor
                        border.color: Qt.darker(background.color, 1.2)
                        // display border only if the option is highlighted
                        border.width: wrapper.isCurrent ? 1 : 0
                        opacity: hoverArea.containsMouse ? 0.3 : 1

                        Behavior on opacity {
                            NumberAnimation { duration: 150 }
                        }
                    }

                    Text {
                        anchors.centerIn: parent

                        text: name

                        color: "black"
                    }

                    MouseArea {
                        id: hoverArea

                        anchors.fill: parent

                        hoverEnabled: true

                        onClicked: floorList.currentIndex = index
                    }
                }
            }
        }
    }
}