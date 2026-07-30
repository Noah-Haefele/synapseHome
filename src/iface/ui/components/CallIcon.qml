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
        text: root.label

        font.pixelSize: parent.width * 0.1
        
        color: root.labelColor

        x: parent.width * 0.56
        y: parent.height * 0.22
    }
}