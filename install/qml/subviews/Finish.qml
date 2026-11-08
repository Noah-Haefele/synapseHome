import QtQuick
import sHInstaller
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
            text: "Installation complete"
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
            text: "synapseHome has been successfully installed on your device."
            font.pixelSize: 16
            color: Theme.colors.secondaryText
            wrapMode: Text.WordWrap
        }

        Text {
            width: parent.width
            text: "The installation process is now complete. Your device is ready to use synapseHome. Depending on your selected configuration, the system services will start automatically on the next boot. The application is already available in the application hub and can also be launched directly from here."
            font.pixelSize: 15
            color: Theme.colors.tertiaryText
            wrapMode: Text.WordWrap
            lineHeight: 1.3
        }

        Text {
            text: "What's next?"
            font.pixelSize: 16
            font.weight: Font.Medium
            color: Theme.colors.text
        }

        Text {
            width: parent.width
            text: "You can now start using synapseHome. For documentation, configuration options and further information, visit the synapseHome GitHub page."
            font.pixelSize: 15
            color: Theme.colors.tertiaryText
            wrapMode: Text.WordWrap
            lineHeight: 1.3
        }
    }

    Row {
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.rightMargin: 35
        anchors.bottomMargin: 30

        spacing: 12

        ModernButton {
            text: "Launch"
            textColor: Theme.colors.oppositeText

            backgroundColor: Theme.colors.green
            width: 140
            height: 48

            // Launch synapseHome and close installer
        }

        ModernButton {
            text: "Finish"
            textColor: Theme.colors.oppositeText

            backgroundColor: Theme.colors.primaryAccent
            width: 140
            height: 48

            // Close installer
        }
    }
}
