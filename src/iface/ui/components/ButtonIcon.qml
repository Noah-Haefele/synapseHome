import QtQuick 2.15
import QtQuick.Controls 2.15

Control {
    id: root

    property url icon: ""
    property color clr: "transparent"

    signal clicked()
    signal pressedAndHold()

    background: Rectangle {
        color: root.clr
    }

    Image {
        anchors.centerIn: parent
        width: parent.width * 0.6
        height: parent.height * 0.6
        fillMode: Image.PreserveAspectFit
        source: Qt.resolvedUrl(root.icon)
    }

    MouseArea {
        anchors.fill: parent

        onClicked: root.clicked()
        onPressAndHold: root.pressedAndHold()
    }
}