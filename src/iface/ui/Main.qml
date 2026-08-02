import QtQuick 2.15
import QtQuick.Controls 2.15

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

    // OutgoingCallOverlay overlay
    Loader {
        id: outgoingCallOverlay
        anchors.fill: parent
        source: "views/call/Outgoing.qml"
        active: false
        z: 1000
    }

    // IncomingCallOverlay overlay
    Loader {
        id: incomingCallOverlay
        anchors.fill: parent
        source: "views/call/Incoming.qml"
        active: false
        z: 1000
    }

    // AcceptedCallOverlay overlay
    Loader {
        id: acceptedCallOverlay
        anchors.fill: parent
        source: "views/call/Accepted.qml"
        active: false
        z: 1000
    }

    Connections {
        target: callHandler

        function onCallStateChanged(state) {
            outgoingCallOverlay.active =
                (state === "CALLING")
            
            incomingCallOverlay.active =
                (state === "RINGING")

            acceptedCallOverlay.active =
                (state === "CONNECTED")
        }
    }
}