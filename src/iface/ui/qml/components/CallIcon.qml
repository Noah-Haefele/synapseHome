import QtQuick

Item {
    id: root

    property url icon: ""
    property string label: ""
    property color labelColor: "black"

    Item {
        id: iconBox
        // Keep the visible icon area square and centered so the label stays
        // anchored to the same visual region when the button is resized.
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
            // Show a fallback marker for labels that are too long to fit nicely.
            text: root.label.length > 2 ? "!!" : root.label

            font.pixelSize: {
                switch (root.label.length) {
                case 2:
                    return iconBox.width * 0.16
                default:
                    return iconBox.width * 0.2
                }
            }

            // Paint oversized labels in red to make overflow obvious.
            color: root.label.length > 2 ? '#ff0000' : root.labelColor

            // Position the label relative to the fitted icon box so it stays
            // visually attached to the icon when the control scales.
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.leftMargin: parent.width * 0.59
            anchors.topMargin: parent.height * 0.20
        }
    }
}