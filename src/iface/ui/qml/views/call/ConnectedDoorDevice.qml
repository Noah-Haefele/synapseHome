import QtQuick 2.15
import QtQuick.Controls 2.15
import CallBridge
import UiBridge
import "../../components"

Rectangle {
    id: root

    anchors.fill: parent

    color: Theme.colors.primaryBackground

    Label {
        anchors {
            horizontalCenter: parent.horizontalCenter
            top: parent.top
            topMargin: 25
        }

        font.pixelSize: 30
        color: Theme.colors.text

        text: CallBridge.destinationLabel
    }

    Row {
        anchors {
            horizontalCenter: parent.horizontalCenter
            bottom: parent.bottom
            bottomMargin: 10
        }

        spacing: 16

        ButtonIcon {
            height: 120
            width: height

            icon: "qrc:/qt/qml/UiBridge/assets/icons/call/muteMic_on.svg"
        }

        ButtonIcon {
            height: 120
            width: height

            icon: "qrc:/qt/qml/UiBridge/assets/icons/call/call_off.svg"

            onClicked: CallBridge.endCall()
        }

        ButtonIcon {
            height: 120
            width: height

            onClicked: CallBridge.openDoor()

            icon: "qrc:/qt/qml/UiBridge/assets/icons/call/openDoor.svg"
        }

        ButtonIcon {
            height: 120
            width: height

            icon: "qrc:/qt/qml/UiBridge/assets/icons/call/muteMic_off.svg"
        }
    }
}
