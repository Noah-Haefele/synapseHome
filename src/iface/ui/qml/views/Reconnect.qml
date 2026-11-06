import QtQuick
import QtQuick.Layouts
import "../components"

Rectangle {
    id: root

    anchors.fill: parent

    color: "white"

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

        Text {
            Layout.alignment: Qt.AlignHCenter

            text: "The backend seems to be offline"

            color: "black"
            font.pixelSize: 23
        }

        Column {
            Layout.alignment: Qt.AlignHCenter
            spacing: 10

            LoadingCircle {
                id: spinner
                running: true
                color: "#79baff"
                lineWidth: 14
                width: 100
                height: 100
            }

            Text {
                anchors.horizontalCenter: parent.horizontalCenter
                text: "Reconnecting ..."
                color: "black"
                font.pixelSize: 12
            }
        }
    }
}
