import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import sHInstaller
import "components"
import "subviews"

ApplicationWindow {
    id: mainWindow

    width: 900
    height: 600

    minimumWidth: 800
    minimumHeight: 550

    visible: true
    title: "Synapse Home Installer"

    color: Theme.colors.secondaryBackground

    RowLayout {
        id: root

        anchors.fill: parent

        spacing: 30

        property int currentStep: 2

        // Sidebar
        Column {
            Layout.fillHeight: true

            Layout.topMargin: 30
            Layout.leftMargin: 30

            spacing: 0

            WizardStep {
                stepNumber: 1
                title: "Welcome"
                currentStep: root.currentStep
            }

            WizardStep {
                stepNumber: 2
                title: "Device Selection"
                currentStep: root.currentStep
            }

            WizardStep {
                stepNumber: 3
                title: "Services"
                currentStep: root.currentStep
            }

            WizardStep {
                stepNumber: 4
                title: "Finish"
                currentStep: root.currentStep
            }
        }
        
        // Main Content
        Welcome {
            Layout.fillHeight: true
            Layout.fillWidth: true

            visible: root.currentStep === 1
        }

        DeviceSelection {
            Layout.fillHeight: true
            Layout.fillWidth: true

            visible: root.currentStep === 2
        }
    }
}
