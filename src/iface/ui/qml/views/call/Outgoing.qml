import QtQuick 2.15
import QtQuick.Controls 2.15
import CallBridge
import "../../components"

Rectangle {
    id: root

    anchors.fill: parent

    color: "white"

    Label {
        anchors {
            horizontalCenter: parent.horizontalCenter
            top: parent.top
            topMargin: 25
        }

        font.pixelSize: 30

        text: CallBridge.destinationLabel
    }

    ButtonIcon {
        anchors {
            horizontalCenter: parent.horizontalCenter
            bottom: parent.bottom
            bottomMargin: 10
        }

        height: 120
        width: height

        icon: "qrc:/qt/qml/UiBridge/assets/icons/call/call_off.svg"

        onClicked: CallBridge.endCall()
    }
}
