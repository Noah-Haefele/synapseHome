import QtQuick
import QtQuick.Controls

ApplicationWindow {
    id: mainWindow

    width: 800
    height: 480
    visible: true
    title: "Synapse Home Installer"

    // Navigation between different views
    StackView {
        id: stackView

        anchors.fill: parent
        initialItem: "Start.qml"
    }
}
