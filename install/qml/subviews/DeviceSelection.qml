import QtQuick
import sHInstaller
import "../components"

Rectangle {
    color: Theme.colors.primaryBackground

    property string selectedDevice: ""

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.top: parent.top
        anchors.topMargin: 30
        width: parent.width * 0.75

        spacing: 18

        Text {
            text: "Select your device"
            font.pixelSize: 30
            font.weight: Font.DemiBold
            color: Theme.colors.text
        }

        Rectangle {
            width: 55
            height: 4
            color: Theme.colors.blue
            radius: 2
        }

        Text {
            width: parent.width
            text: "Please select the type of device you want synapseHome to install."
            font.pixelSize: 16
            color: Theme.colors.secondaryText
            wrapMode: Text.WordWrap
        }

        Text {
            width: parent.width
            text: "Choose whether this installation is for a GUI device or a door device. The configuration and required services will be adjusted accordingly. For more information visit the synapseHome GitHub page."
            font.pixelSize: 15
            color: Theme.colors.tertiaryText
            wrapMode: Text.WordWrap
            lineHeight: 1.3
        }

        Text {
            text: "Device type"
            font.pixelSize: 16
            font.weight: Font.Medium
            color: Theme.colors.text
        }

        Column {
            width: parent.width
            spacing: 12

            RadioButton {
                text: "Touch Device"
                checked: selectedDevice === "touch"

                onClicked: selectedDevice = "touch"
            }

            RadioButton {
                text: "Door Device"
                checked: selectedDevice === "door"

                onClicked: selectedDevice = "door"
            }
        }
    }

    ModernButton {
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.rightMargin: 35
        anchors.bottomMargin: 30

        text: "Install"
        symbol: "\u2192"
        textColor: Theme.colors.oppositeText

        backgroundColor: Theme.colors.primaryAccent
        width: 140
        height: 48
    }
}
