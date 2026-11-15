import QtQuick
import sHInstaller
import GuiInstaller
import "../components"

Rectangle {
    id: root

    color: Theme.colors.primaryBackground

    property string selectedDevice: ""
    property bool error: false

    function validatePageSwitch() {
        if (root.selectedDevice === "") {
            root.error = true
            errorAnimation.restart()
            return false
        }

        return true
    }

    function selectDevice(device) {
        root.selectedDevice = device
        root.error = false
        errorAnimation.stop()
        errorRectangle.border.color = "transparent"
    }

    SequentialAnimation {
        id: errorAnimation

        ColorAnimation {
            target: errorRectangle.border
            property: "color"
            from: "transparent"
            to: Theme.colors.error
            duration: 300
            easing.type: Easing.OutCubic
        }

        PauseAnimation {
            duration: 400
        }

        ColorAnimation {
            target: errorRectangle.border
            property: "color"
            from: Theme.colors.error
            to: "transparent"
            duration: 500
            easing.type: Easing.InOutQuad
        }
    }

    Column {
        anchors.fill: parent
        anchors.topMargin: 30
        spacing: 10
        
        Column {
            id: textColumn
            
            width: parent.width * 0.75

            anchors.horizontalCenter: parent.horizontalCenter

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

            Row {
                spacing: 20

                Text {
                    text: "Device type"
                    font.pixelSize: 16
                    font.weight: Font.Medium
                    color: Theme.colors.text
                }

                Text {
                    text: "Input Required!"
                    font.pixelSize: 16
                    font.bold: true
                    color: Theme.colors.error

                    visible: error
                }
            }
        }
        
        Rectangle {
            id: errorRectangle

            anchors.horizontalCenter: parent.horizontalCenter

            width: textColumn.width + 20
            height: buttonColumn.implicitHeight + 20

            radius: 8
            border.width: 2
            border.color: "transparent"

            Column {
                id: buttonColumn

                width: parent.width - 20
                anchors.centerIn: parent
                spacing: 12

                RadioButton {
                    text: "Touch Device"
                    checked: selectedDevice === "touch"

                    onClicked: root.selectDevice("touch")
                }

                RadioButton {
                    text: "Door Device"
                    checked: selectedDevice === "door"

                    onClicked: root.selectDevice("door")
                }
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

        onClicked: {
            if (validatePageSwitch()) {
                switch (root.selectedDevice) {
                    case "touch":
                        GuiInstaller.installDevice(GuiInstaller.GuiDevice)
                        break
                    case "door":
                        GuiInstaller.installDevice(GuiInstaller.DoorDevice)
                        break
                }
                GuiInstaller.nextInstallationPage()
            }
        }
    }
}
