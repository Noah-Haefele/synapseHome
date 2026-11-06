import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import UiBridge
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

            backgroundColor: Theme.colors.primaryBackground
            textColor: Theme.colors.text
            primaryAccentColor: Theme.colors.primaryAccent
            secondaryAccentColor: Theme.colors.secondaryBackground

            onBackClicked: stackView.pop()
            onOptionClicked: (index, name) => {
                root.leftViewState = index
            }
        }

        // Border between navigation panel and content area
        Rectangle {
            height: parent.height
            width: 1

            color: Theme.colors.greySeperator
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

            Audio {
                visible: leftViewState === 1
                Layout.fillWidth: true
                Layout.fillHeight: true
            }

            Ringtone {
                visible: leftViewState === 2
                Layout.fillWidth: true
                Layout.fillHeight: true
            }

            Rectangle {
                visible: leftViewState !== 3 && leftViewState !== 0 && leftViewState !== 1 && leftViewState !== 2
                Layout.fillHeight: true
                Layout.fillWidth: true
                color: Theme.colors.primaryBackground

                Label {
                    anchors.centerIn: parent
                    text: "Settings Content"
                    color: Theme.colors.text
                }
            }
        }
    }
}
