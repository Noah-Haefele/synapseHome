import QtQuick

Item {
    id: root

    property string text: "Next"
    property string symbol: ""
    property color textColor: "#FFFFFF"
    property color backgroundColor: "#4F46E5"

    signal clicked()

    implicitWidth: 150
    implicitHeight: 48

    width: implicitWidth
    height: implicitHeight

    Rectangle {
        id: buttonBackground
        anchors.fill: parent
        radius: 12
        color: root.backgroundColor

        scale: mouseArea.pressed ? 0.97 : 1.0

        Behavior on scale {
            NumberAnimation {
                duration: 100
                easing.type: Easing.OutCubic
            }
        }

        Behavior on color {
            ColorAnimation { duration: 150 }
        }

        Row {
            anchors.centerIn: parent
            spacing: 12

            Text {
                text: root.text
                color: root.textColor
                font.pixelSize: 15
                font.weight: Font.DemiBold
                anchors.verticalCenter: parent.verticalCenter
            }

            Text {
                text: root.symbol
                color: root.textColor
                font.pixelSize: 21
                anchors.verticalCenter: parent.verticalCenter
            }
        }
    }

    MouseArea {
        id: mouseArea
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor

        onClicked: root.clicked()
    }

    Rectangle {
        anchors.fill: parent
        radius: 12
        color: "white"
        opacity: mouseArea.containsMouse ? 0.08 : 0
        visible: !mouseArea.pressed

        Behavior on opacity {
            NumberAnimation { duration: 150 }
        }
    }
}
