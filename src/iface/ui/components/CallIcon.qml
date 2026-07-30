import QtQuick

Item {
    id: root

    property url icon: ""
    property string label: ""
    property color labelColor: "black"

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
                return parent.width * 0.08
            // Number to large
            default:
                return parent.width * 0.1
            }
        }
        
        // Paint double exclamaiton mark in red
        color: root.label.length > 2 ? '#ff0000' : root.labelColor

        x: {
            switch (root.label.length) {
            case 2:
                return parent.width * 0.54
            default:
                return parent.width * 0.56
            }
        }
        y: parent.height * 0.23
    }
}