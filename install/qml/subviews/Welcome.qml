import QtQuick
import sHInstaller
import GuiInstaller
import "../components"

Rectangle {
    color: Theme.colors.primaryBackground

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.top: parent.top
        anchors.topMargin: 30
        width: parent.width * 0.75

        spacing: 18

        Text {
            text: "Welcome to synapseHome"
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
            text: "This wizard will guide you through the installation of synapseHome."
            font.pixelSize: 16
            color: Theme.colors.secondaryText
            wrapMode: Text.WordWrap
        }

        Text {
            width: parent.width
            text: "You will be able to select your device type, configure the required services and complete the installation."
            font.pixelSize: 15
            color: Theme.colors.tertiaryText
            wrapMode: Text.WordWrap
            lineHeight: 1.3
        }

        Text {
            text: "Click Next to continue."
            font.pixelSize: 14
            font.weight: Font.Medium
            color: Theme.colors.text
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

        onClicked: GuiInstaller.nextInstallationPage()
    }
}
