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

            icon: "qrc:/qt/qml/UiBridge/assets/icons/call/call_on.svg"

            onClicked: callHandler.acceptCall()
        }

        ButtonIcon {
            height: 120
            width: height

            icon: "qrc:/qt/qml/UiBridge/assets/icons/call/call_off.svg"

            onClicked: callHandler.endCall()
        }
    }
}
