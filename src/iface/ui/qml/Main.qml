import QtQuick 2.15
import QtQuick.Controls 2.15
import CallBridge

ApplicationWindow {
    id: mainWindow

    width: 800
    height: 480
    visible: true
    title: "Synapse UI"

    // Navigation between different views
    StackView {
        id: stackView

        anchors.fill: parent
        initialItem: "views/Home.qml"
    }

    Item {
        id: overlayLayer

        anchors.fill: parent

        z: 1000
        visible: false

        MouseArea {
            anchors.fill: parent

            enabled: overlayLayer.visible
        }

        Loader {
            id: callOverlayLoader

            anchors.fill: parent
        }
    }

    Connections {
        target: CallBridge

        function onCallStateChanged(state) {
            overlayLayer.visible = state !== "IDLE"

            if (state === "CALLING") {
                callOverlayLoader.source = "views/call/Outgoing.qml"
            }
            else if (state === "RINGING") {
                callOverlayLoader.source = "views/call/Incoming.qml"
            }
            else if (state === "CONNECTED") {
                callOverlayLoader.source = "views/call/Accepted.qml"
            }
        }
    }
}
