import QtQuick 2.15
import QtQuick.Controls 2.15

ComboBox {
    id: root

    height: 60
    width: 300

    font.pixelSize: 24
    font.family: "DejaVu Sans, sans-serif"

    // Ensure a default selection is made when the model is loaded 
    // dynamically, preventing the ComboBox from getting stuck on index -1
    onCountChanged: {
        if (count > 0 && currentIndex === -1) {
            currentIndex = 0
        }
    }

    // arrow
    indicator: Text {
        x: root.width - width - 20
        anchors.verticalCenter: parent.verticalCenter
        text: root.popup.visible ? "▲" : "▼"
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
        text: root.displayText
        
        color: "#333333"
        font: root.font

        verticalAlignment: Text.AlignVCenter

        leftPadding: 20
        // prevents overlapping with arrow
        rightPadding: root.indicator ? root.indicator.width + 30 : 20

        elide: Text.ElideRight
    }

    popup: Popup {
        y: root.height + 8
        width: root.width

        padding: 0

        background: Rectangle {
            color: "#ffffff"
            radius: 12

            border.color: "#e0e0e0"
            border.width: 1
        }

        // available options
        contentItem: ListView {
            model: root.delegateModel

            clip: true
            spacing: 0
            
            implicitHeight: Math.min(contentHeight, 400)

            delegate: ItemDelegate {
                width: root.width
                height: 70

                text: root.textRole !== "" ? model[root.textRole] : modelData

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