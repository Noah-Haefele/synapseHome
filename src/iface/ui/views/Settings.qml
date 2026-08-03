import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../components"
import "../subviews/settings"

Item {
    id: root

    // For right subviews:
    // 0 = display
    // 1 = audio
    // 2 = ring-tone
    // 3 = system
    // 4 = outdoor-station
    // 5 = utility-server
    property int leftViewState: 0

    RowLayout {
        anchors.fill: parent
        spacing: 0

        // Navigation panel
        NavBar {
            preferredWidth: parent.width * 0.25

            onBackClicked: stackView.pop()
            onOptionClicked: (index, name) => {
                root.leftViewState = index
            }
        }

        // Border between navigation panel and content area
        Rectangle {
            height: parent.height
            width: 1

            color: '#dbdbdd'
        }

        // Content area
        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 0

            Display {
                visible: leftViewState === 0
                Layout.fillWidth: true
                Layout.fillHeight: true
            }

            System {
                visible: leftViewState === 3
                Layout.fillWidth: true
                Layout.fillHeight: true
            }

            Rectangle {
                visible: leftViewState !== 3 && leftViewState !== 0
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
}