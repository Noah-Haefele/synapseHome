import QtQuick
import QtQuick.Layouts
import "../components"

Rectangle {
    id: root

    anchors.fill: parent

    color: "white"

    RowLayout {
        anchors.fill: parent
        spacing: 0

        LoadingCircle {
            id: spinner
            running: true
            color: "#79baff"
            lineWidth: 14
            
            Layout.preferredWidth: 100
            Layout.preferredHeight: 100
            
            Layout.alignment: Qt.AlignCenter 
        }
    }
}
