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

    // Call overlay
    Loader {
        id: callOverlay
        anchors.fill: parent
        source: "views/call/Outgoing.qml"
        active: false
        z: 1000
    }

    Connections {
        target: callHandler

        function onCallStateChanged(state) {
            callOverlay.active =
                (state === "CALLING")
        }
    }
}