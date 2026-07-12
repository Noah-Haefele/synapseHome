import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../components"

Rectangle {
    id: root
    
    color: "black"

    signal singleClicked(string name)

    ListModel {
        id: buttonData

        ListElement {
            name: "floorD"
            iconPath: "../../../../assets/icons/button/callD.svg"
        }

        ListElement {
            name: "mute"
            iconPath: ""
        }

        ListElement {
            name: "floor1"
            iconPath: "../../../../assets/icons/button/call1.svg"
        }

        ListElement {
            name: "light"
            iconPath: ""
        }

        ListElement {
            name: "floorE"
            iconPath: "../../../../assets/icons/button/callE.svg"
        }

        ListElement {
            name: "empty"
            iconPath: ""
        }

        ListElement {
            name: "floorA"
            iconPath: "../../../../assets/icons/button/callA.svg"
        }
        
        ListElement {
            name: "settings"
            iconPath: "../../../../assets/icons/button/settings.svg"
        }
    }

    GridLayout {
        anchors.fill: parent
        columns: 2
        rows: 4
        columnSpacing: 2
        rowSpacing: 2

        Repeater {
            model: buttonData

            ButtonIcon {
                Layout.fillWidth: true
                Layout.fillHeight: true

                icon: model.iconPath

                clr: "white"

                onClicked: root.singleClicked(model.name)
            }
        }
    }
}