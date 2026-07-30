import QtQuick 2.15
import QtQuick.Controls 2.15

Control {
    id: root

    property color clr: "transparent"

    // CallIcon relevant stuff
    property url icon: ""
    property string label: ""
    property color labelColor: "black"

    signal clicked()
    signal pressedAndHold()

    background: Rectangle {
        color: root.clr
    }

    CallIcon {
        anchors.centerIn: parent

        width: parent.width * 0.6
        height: parent.height * 0.6

        icon: root.icon
        label: root.label
        labelColor: root.labelColor
    }

    MouseArea {
        anchors.fill: parent

        onClicked: root.clicked()
        onPressAndHold: root.pressedAndHold()
    }
}