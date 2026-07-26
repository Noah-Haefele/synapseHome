import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../components"
import "../models"

Rectangle {
    id: root
    
    color: "black"

    signal singleClicked(string type, var value)

    // Create instance of Floors model
    Floors {
        id: floors
    }

    /*
    Button layout configuration.

    This model defines the order and type of buttons displayed in the GridLayout.
    Floor-specific data (name, icon, floor index) is stored separately in Floors.qml.

    Floor buttons use floorId to reference the corresponding entry in the Floors model.
    This avoids duplicating floor information while keeping the UI layout flexible.

    When adding a new floor:
    1. Add the floor data to Floors.qml.
    2. Add a button entry here with type "floor" and the matching floorIdx.
    */
    ListModel {
        id: buttonData

        ListElement {
            type: "floor"
            floorIdx: 1
        }

        ListElement {
            name: "mute"
            type: "misc"
            iconPath: ""
        }

        ListElement {
            type: "floor"
            floorIdx: 2
        }

        ListElement {
            name: "light"
            type: "misc"
            iconPath: ""
        }

        ListElement {
            type: "floor"
            floorIdx: 3
        }

        ListElement {
            name: "empty"
            type: "misc"
            iconPath: ""
        }

        ListElement {
            name: "floorA"
            type: "misc"
            iconPath: "../../../../assets/icons/button/callA.svg"
        }
        
        ListElement {
            name: "settings"
            type: "misc"
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

                icon: {
                    if (model.type === "floor") {
                        switch (model.floorIdx) {
                        case 1:
                            return uiHandler.pref1IconPath
                        case 2:
                            return uiHandler.pref2IconPath
                        case 3:
                            return uiHandler.pref3IconPath
                        default:
                            return ""
                        }
                    }
                    return model.iconPath
                }

                clr: "white"

                onClicked: {
                    root.singleClicked(
                        model.type,
                        model.type === "floor" ? model.floorIdx : model.name
                    )
                }
            }
        }
    }
}