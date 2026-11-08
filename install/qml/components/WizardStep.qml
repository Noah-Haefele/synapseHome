import QtQuick
import QtQuick.Controls
import sHInstaller

Row {
    id: root

    property int stepNumber: 1
    property string title: ""
    property int currentStep: 1

    spacing: 12

    Column {
        spacing: 0

        Rectangle {
            width: 28
            height: 28
            radius: 14

            color: {
                if (root.stepNumber < root.currentStep)
                    return Theme.colors.primaryAccent
                if (root.stepNumber === root.currentStep)
                    return Theme.colors.blue
                return Theme.colors.blackSeperator
            }

            Text {
                anchors.centerIn: parent

                text: root.stepNumber < root.currentStep
                      ? "\u{1F5F8}" // Unicode for check mark
                      : root.stepNumber

                color: Theme.colors.oppositeText
            }
        }

        Rectangle {
            width: 2
            height: 35
            color: Theme.colors.primaryBackground
            anchors.horizontalCenter: parent.horizontalCenter
            visible: root.stepNumber < 5
        }
    }

    Text {
        anchors.verticalCenter: parent.verticalCenter

        text: root.title

        color: root.stepNumber === root.currentStep
               ? Theme.colors.text
               : Theme.colors.blackSeperator
    }
}
