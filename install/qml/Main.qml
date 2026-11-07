import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "components"
import "subviews"

ApplicationWindow {
    id: mainWindow

    width: 800
    height: 480
    visible: true
    title: "Synapse Home Installer"

    RowLayout {
        anchors.fill: parent
        spacing: 30

        // Sidebar
        Column {
            id: left

            Layout.fillHeight: true

            Layout.topMargin: 30
            Layout.leftMargin: 30

            spacing: 0

            property int currentStep: 3

            WizardStep {
                stepNumber: 1
                title: "Welcome"
                currentStep: parent.currentStep
            }

            WizardStep {
                stepNumber: 2
                title: "Device Selection"
                currentStep: parent.currentStep
            }

            WizardStep {
                stepNumber: 3
                title: "Services"
                currentStep: parent.currentStep
            }

            WizardStep {
                stepNumber: 4
                title: "Finish"
                currentStep: parent.currentStep
            }
        }

        // Main Content
        Welcome {
            Layout.fillHeight: true
            Layout.fillWidth: true

            visible: true
        }
            
    }
}
