import QtQuick
import sHInstaller

Rectangle {
    id: root

    property string text: ""
    property bool checked: false

    signal clicked()

    width: parent ? parent.width : 300
    height: 65
    radius: 8

    // Sets a blue background with 20% opacity when checked; otherwise, the background is transparent.
    color: {
        let c = Qt.color(checked ? Theme.colors.blue : Theme.colors.primaryBackground)
        return Qt.rgba(c.r, c.g, c.b, (checked ? 0.2 : 0))
    }

    border.width: checked ? 2 : 1
    border.color: checked ? Theme.colors.blue : Theme.colors.secondaryAccent

    Row {
        anchors.fill: parent
        anchors.leftMargin: 18
        spacing: 14

        Rectangle {
            width: 20
            height: 20
            radius: 10
            anchors.verticalCenter: parent.verticalCenter

            color: "transparent"
            border.width: 2
            border.color: checked ? Theme.colors.blue : Theme.colors.secondaryAccent

            Rectangle {
                anchors.centerIn: parent
                width: 10
                height: 10
                radius: 5
                color: Theme.colors.blue
                visible: checked
            }
        }

        Text {
            text: root.text
            anchors.verticalCenter: parent.verticalCenter
            font.pixelSize: 16
            font.weight: Font.Medium
            color: Theme.colors.text
        }
    }

    MouseArea {
        anchors.fill: parent
        cursorShape: Qt.PointingHandCursor

        onClicked: root.clicked()
    }
}
