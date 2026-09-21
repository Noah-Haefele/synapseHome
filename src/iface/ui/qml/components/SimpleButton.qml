import QtQuick
import QtQuick.Controls

Button {
    id: button

    property color backgroundColor: "transparent"
    property color textColor: "black"
    property int radius: 0
    property int pixelsize: 12

    background: Rectangle {
        radius: button.radius
        color: button.hovered ? Qt.darker(button.backgroundColor, 1.2) : button.backgroundColor
    }

    contentItem: Text {
        text: button.text

        color: button.textColor
        font.pixelSize: button.pixelsize

        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
}
