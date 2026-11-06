import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    property string label: "Label"
    property int val: 50
    property string unit: "%"
    property int min: 0
    property int max: 100
    signal moved(int value)

    property color textColor: "black"
    property color valueColor: "black"
    property color primaryAccentColor: "#79baff"
    property color secondaryAccentColor: "lightgray"
    property color knobColor: "white"

    height: 80
    width: parent.width

    Column {
        anchors.fill: parent

        spacing: 8

        Item {
            height: 30
            width: parent.width

            Text {
                anchors.left: parent.left

                text: root.label

                font.pixelSize: 22
                font.bold: true
                color: root.textColor
            }
            Text {
                anchors.right: parent.right

                text: root.val + " " + root.unit

                font.pixelSize: 22
                font.bold: true
                color: root.valueColor
            }
        }

        // Settings slider
        Slider {
            id: control

            width: parent.width

            from: root.min
            to: root.max
            value: root.val
            onMoved: root.moved(Math.round(control.value))

            background: Rectangle {
                x: control.leftPadding
                y: control.topPadding + control.availableHeight / 2 - height / 2

                implicitWidth: 200
                implicitHeight: 12
                width: control.availableWidth
                height: implicitHeight

                radius: 6
                color: root.secondaryAccentColor

                Rectangle {
                    width: control.visualPosition * parent.width
                    height: parent.height

                    color: root.primaryAccentColor
                    radius: 6
                }
            }

            handle: Rectangle {
                x: control.leftPadding + control.visualPosition * (control.availableWidth - width)
                y: control.topPadding + control.availableHeight / 2 - height / 2

                implicitWidth: 50
                implicitHeight: 50

                radius: 25
                color: root.knobColor
                border.color: root.primaryAccentColor
                border.width: 3
            }
        }
    }
}
