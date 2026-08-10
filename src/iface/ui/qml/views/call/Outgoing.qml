import QtQuick 2.15
import QtQuick.Controls 2.15
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

        text: callHandler.destinationLabel
    }

    ButtonIcon {
        anchors {
            horizontalCenter: parent.horizontalCenter
            bottom: parent.bottom
            bottomMargin: 10
        }

        height: 120
        width: height

        icon: "../../../../assets/icons/call/call_off.svg"

        onClicked: callHandler.endCall()
    }
}