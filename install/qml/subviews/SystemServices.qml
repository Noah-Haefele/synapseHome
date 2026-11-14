import QtQuick
import sHInstaller
import GuiInstaller
import "../components"

Rectangle {
    id: root

    color: Theme.colors.primaryBackground

    property string selectedService: ""
    property bool guiDevice: GuiInstaller.isGuiDevice
    property bool error: false

    function validatePageSwitch() {
        if (root.selectedService === "") {
            root.error = true
            errorTimer.restart()
            errorRectangle.border.color = Theme.colors.error
            return false
        }

        return true
    }

    function selectServices(service) {
        root.selectedService = service
        root.error = false
        errorTimer.stop()
        errorRectangle.border.color = "transparent"
    }

    Timer {
        id: errorTimer
        interval: 1000
        repeat: false
        running: false

        onTriggered: {
            errorRectangle.border.color = "transparent"
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
                visible: !root.guiDevice

                width: parent.width
                text: "Choose whether the backend daemon (synapsed) should run automatically as a system service. This determines how synapseHome operates in the background. For more information, visit the synapseHome GitHub page."
                font.pixelSize: 15
                color: Theme.colors.tertiaryText
                wrapMode: Text.WordWrap
                lineHeight: 1.3
            }

            // Installation with GUI
            Text {
                visible: root.guiDevice

                width: parent.width
                text: "Choose whether the backend daemon (synapsed) should run automatically as a system service, either alone or together with the graphical interface (synapse UI). The selected components will be configured accordingly. For more information, visit the synapseHome GitHub page."
                font.pixelSize: 15
                color: Theme.colors.tertiaryText
                wrapMode: Text.WordWrap
                lineHeight: 1.3
            }

            Row {
                spacing: 20

                Text {
                    text: "System services"
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
                    text: "No system services"
                    checked: root.selectedService === "none"

                    onClicked: root.selectServices("none")
                }

                RadioButton {
                    text: "Backend daemon (synapsed)"
                    checked: root.selectedService === "backend"

                    onClicked: root.selectServices("backend")
                }

                // Only available for GUI installations
                RadioButton {
                    visible: root.guiDevice
                    text: "Backend daemon (synapsed) + GUI (synapse UI)"
                    checked: root.selectedService === "backendGui"

                    onClicked: root.selectServices("backendGui")
                }
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

        onClicked: {
            if (validatePageSwitch()) {
                switch (root.selectedService) {
                    case "none":
                        GuiInstaller.service(GuiInstaller.None)
                        break
                    case "backend":
                        GuiInstaller.service(GuiInstaller.Backend)
                        break
                    case "backendGui":
                        GuiInstaller.service(GuiInstaller.BackendGui)
                        break
                }
                GuiInstaller.nextInstallationPage()
            }
        }
    }
}
