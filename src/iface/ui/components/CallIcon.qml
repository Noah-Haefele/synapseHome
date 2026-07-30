import QtQuick

Item {
    id: root

    property url icon: ""
    property string label: ""
    property color labelColor: "black"

    Item {
        id: iconBox
        anchors.centerIn: parent
        width: Math.min(parent.width, parent.height)
        height: width

        Image {
            id: image
            anchors.fill: parent
            fillMode: Image.PreserveAspectFit
            source: Qt.resolvedUrl(root.icon)
        }

        Text {
            // Warning if Number to large
            text: root.label.length > 2 ? "!!" : root.label

            font.pixelSize: {
                switch (root.label.length) {
                case 2:
                    return iconBox.width * 0.16
                default:
                    return iconBox.width * 0.2
                }
            }

            // Paint double exclamaiton mark in red
            color: root.label.length > 2 ? '#ff0000' : root.labelColor

            anchors.left: parent.left
            anchors.top: parent.top
            anchors.leftMargin: parent.width * 0.59
            anchors.topMargin: parent.height * 0.20
        }
    }
}