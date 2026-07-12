import QtQuick
import QtQuick.Controls

Button {
    id: button
    
    background: Rectangle {
        radius: 10
        color: button.down ? "#e8e8e8" : "transparent"
    }

    contentItem: Text {
        text: button.text

        color: "#222"
        font.pixelSize: 12

        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
}