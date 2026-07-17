import QtQuick 2.15
import QtQuick.Controls 2.15
import "../../components"
import "../../models"

Rectangle {
    id: configRoot

    color: "#f8f9fa"

    // Main content
    Column {
        anchors {
            top: parent.top
            horizontalCenter: parent.horizontalCenter
            topMargin: 90
        }

        width: parent.width * 0.8

        spacing: 60

        Dropdown {
            width: 400
            height: 60

            anchors.horizontalCenter: parent.horizontalCenter
            
            model: Floors {}
            textRole: "name"
        }
    }
}