import QtQuick
import QtQuick.Layouts
import UiBridge
import "../components"

Rectangle {
    id: root

    anchors.fill: parent

    color: Theme.colors.primaryBackground

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

        Text {
            Layout.alignment: Qt.AlignHCenter

            text: "The backend seems to be offline"

            color: Theme.colors.text
            font.pixelSize: 23
        }

        Column {
            Layout.alignment: Qt.AlignHCenter
            spacing: 10

            LoadingCircle {
                id: spinner
                running: true
                color: Theme.colors.primaryAccent
                lineWidth: 14
                width: 100
                height: 100
            }

            Text {
                anchors.horizontalCenter: parent.horizontalCenter
                text: "Reconnecting ..."
                color: Theme.colors.text
                font.pixelSize: 12
            }
        }
    }
}
