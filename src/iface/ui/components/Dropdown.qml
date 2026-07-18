import QtQuick 2.15
import QtQuick.Controls 2.15

Column {
    id: root

    property alias model: combo.model
    property alias currentIndex: combo.currentIndex
    property alias textRole: combo.textRole
    property alias count: combo.count

    property alias label: label.text

    // Default width
    width: 350

    spacing: 1

    Text {
        id: label

        visible: text.length > 0
        text: ""

        font.pixelSize: 11

        color: "#6b7280"
    }

    ComboBox {
        id: combo

        height: 60
        width: root.width

        font.pixelSize: 24
        font.family: "DejaVu Sans, sans-serif"

        // Ensure a default selection is made when the model is loaded 
        // dynamically, preventing the ComboBox from getting stuck on index -1
        onCountChanged: {
            if (count > 0 && currentIndex === -1) {
                currentIndex = 0
            }
        }

        // Dropdown arrow indicator
        indicator: Text {
            x: combo.width - width - 20
            anchors.verticalCenter: parent.verticalCenter
            text: combo.popup.visible ? "▲" : "▼"
            color: "#999999"
            font.pixelSize: 18
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
        }

        background: Rectangle {
            color: "#ffffff"
            radius: 12
            border.color: "#e0e0e0"
            border.width: 1
        }

        // content
        contentItem: Text {
            text: combo.displayText
            
            color: "#333333"
            font: combo.font

            verticalAlignment: Text.AlignVCenter

            leftPadding: 20
            // prevents overlapping with arrow
            rightPadding: combo.indicator ? combo.indicator.width + 30 : 20

            elide: Text.ElideRight
        }

        popup: Popup {
            y: combo.height + 10
            width: combo.width

            background: Rectangle {
                color: "#ffffff"
                radius: 12

                border.color: "#e0e0e0"
                border.width: 1
            }

            // available options
            contentItem: ListView {
                model: combo.delegateModel

                clip: true
                spacing: 0
                
                implicitHeight: Math.min(contentHeight, 400)

                delegate: ItemDelegate {
                    width: combo.width
                    height: 70

                    text: combo.textRole !== "" ? model[combo.textRole] : modelData

                    contentItem: Text {
                        text: parent.text

                        color: highlighted ? "#2F4C6B" : "#333333"
                        font.pixelSize: 22

                        verticalAlignment: Text.AlignVCenter

                        leftPadding: 20
                    }

                    background: Rectangle {
                        color: highlighted ? "#f0f0f0" : "#ffffff"
                    }
                }
            }
        }
    }
}
