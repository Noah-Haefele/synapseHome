import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../components"
import "../models"

Rectangle {
    id: root
    
    color: "black"

    signal singleClicked(string type, var value)

    /*
    * Button layout configuration.
    *
    * Defines the order and type of buttons displayed in the GridLayout.
    */
    ListModel {
        id: buttonData

        ListElement {
            type: "floor"
            prefNum: 1
        }

        ListElement {
            name: "mute"
            type: "misc"
            iconPath: ""
        }

        ListElement {
            type: "floor"
            prefNum: 2
        }

        ListElement {
            name: "light"
            type: "misc"
            iconPath: ""
        }

        ListElement {
            type: "floor"
            prefNum: 3
        }

        ListElement {
            name: "empty"
            type: "misc"
            iconPath: ""
        }

        ListElement {
            name: "floorA"
            type: "misc"
        }
        
        ListElement {
            name: "settings"
            type: "misc"
            iconPath: "qrc:/qt/qml/UiBridge/assets/icons/button/settings.svg"
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
                        switch (model.prefNum) {
                        case 1:
                            return uiHandler.pref1IconPath
                        case 2:
                            return uiHandler.pref2IconPath
                        case 3:
                            return uiHandler.pref3IconPath
                        default:
                            return ""
                        }
                    } else if (model.name === "floorA") {
                        return "qrc:/qt/qml/UiBridge/assets/icons/button/call.svg"
                    }
                    return model.iconPath
                }

                clr: "white"

                label: {
                    if (model.type === "floor") {
                        switch (model.prefNum) {
                        case 1:
                            return uiHandler.pref1ShortName
                        case 2:
                            return uiHandler.pref2ShortName
                        case 3:
                            return uiHandler.pref3ShortName
                        default:
                            return ""
                        }
                    } else if (model.name === "floorA") {
                        return "A"
                    }
                    return ""
                }

                onClicked: {
                    root.singleClicked(
                        model.type,
                        model.type === "floor" ? model.prefNum : model.name
                    )
                }
            }
        }
    }
}