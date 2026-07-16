import QtQuick
import QtQuick.Controls

Button {
    id: button

    property color backgroundColor: "transparent"
    property color textColor: "black"
    
    background: Rectangle {
        radius: 8
        opacity: button.down ? 0.3 : 1
        color: button.backgroundColor
        border.color: Qt.darker(button.backgroundColor, 1.2)
        border.width: 1
    }

    contentItem: Text {
        text: button.text

        color: button.textColor
        font.pixelSize: 12

        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
}