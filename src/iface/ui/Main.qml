import QtQuick 2.15
import QtQuick.Controls 2.15

ApplicationWindow {
    id: mainWindow

    width: 800
    height: 480
    visible: true
    title: "Nexus UI"

    // Navigation between different views
    StackView {
        id: stackView

        anchors.fill: parent
        initialItem: "views/Home.qml"
    }
}