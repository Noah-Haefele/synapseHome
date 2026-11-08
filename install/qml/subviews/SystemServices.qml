import QtQuick
import sHInstaller
import "../components"

Rectangle {
    color: Theme.colors.primaryBackground

    property string selectedService: ""
    property bool guiDevice: true

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.top: parent.top
        anchors.topMargin: 30
        width: parent.width * 0.75

        spacing: 18

        Text {
            text: "Configure system services"
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
            text: "Choose how synapseHome should run on your device."
            font.pixelSize: 16
            color: Theme.colors.secondaryText
            wrapMode: Text.WordWrap
        }

        // Installation without GUI
        Text {
            visible: !guiDevice

            width: parent.width
            text: "Choose whether the backend daemon (synapsed) should run automatically as a system service. This determines how synapseHome operates in the background. For more information, visit the synapseHome GitHub page."
            font.pixelSize: 15
            color: Theme.colors.tertiaryText
            wrapMode: Text.WordWrap
            lineHeight: 1.3
        }

        // Installation with GUI
        Text {
            visible: guiDevice

            width: parent.width
            text: "Choose whether the backend daemon (synapsed) should run automatically as a system service, either alone or together with the graphical interface (synapse UI). The selected components will be configured accordingly. For more information, visit the synapseHome GitHub page."
            font.pixelSize: 15
            color: Theme.colors.tertiaryText
            wrapMode: Text.WordWrap
            lineHeight: 1.3
        }

        Text {
            text: "System services"
            font.pixelSize: 16
            font.weight: Font.Medium
            color: Theme.colors.text
        }

        Column {
            width: parent.width
            spacing: 12

            RadioButton {
                text: "No system services"
                checked: selectedService === "none"

                onClicked: selectedService = "none"
            }

            RadioButton {
                text: "Backend daemon (synapsed)"
                checked: selectedService === "backend"

                onClicked: selectedService = "backend"
            }

            // Only available for GUI installations
            RadioButton {
                visible: guiDevice
                text: "Backend daemon (synapsed) + GUI (synapse UI)"
                checked: selectedService === "backendgui"

                onClicked: selectedService = "backendgui"
            }
        }
    }

    ModernButton {
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.rightMargin: 35
        anchors.bottomMargin: 30

        text: "Next"
        symbol: "\u2192"
        textColor: Theme.colors.oppositeText

        backgroundColor: Theme.colors.primaryAccent
        width: 140
        height: 48
    }
}
