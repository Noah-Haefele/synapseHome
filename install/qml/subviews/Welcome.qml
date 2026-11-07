import QtQuick
import sHInstaller
import "../components"

Rectangle {
    color: "#FFFFFF"

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
            color: "#202020"
        }

        Rectangle {
            width: 55
            height: 4
            color: "#3B82F6"
            radius: 2
        }

        Text {
            width: parent.width
            text: "This wizard will guide you through the installation of synapseHome."
            font.pixelSize: 16
            color: "#444444"
            wrapMode: Text.WordWrap
        }

        Text {
            width: parent.width
            text: "You will be able to select your device type, configure the required services and complete the installation."
            font.pixelSize: 15
            color: "#666666"
            wrapMode: Text.WordWrap
            lineHeight: 1.3
        }

        Text {
            text: "Click Next to continue."
            font.pixelSize: 14
            font.weight: Font.Medium
            color: "#202020"
        }
    }
        
    ModernButton {
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.rightMargin: 35
        anchors.bottomMargin: 30

        text: "Start"
        textColor: "white"

        backgroundColor: Theme.colors.primaryAccent
        width: 140
        height: 48
    }
}
