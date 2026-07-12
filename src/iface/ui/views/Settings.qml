import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../components"

Item {
    id: root

    RowLayout {
        anchors.fill: parent
        spacing: 0

        // Navigation panel
        NavBar {
            preferredWidth: parent.width * 0.25
            
            onBackClicked: stackView.pop()
        }

        // Content area
        Rectangle {
            Layout.fillHeight: true
            Layout.fillWidth: true
            color: "white"

            Label {
                anchors.centerIn: parent
                text: "Settings Content"
            }
        }
    }
}