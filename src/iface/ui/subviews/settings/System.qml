import QtQuick 2.15
import QtQuick.Controls 2.15
import "../../components"
import "../../models"

/**
 * System-Settings Screen View
 * 
 * Provides UI controls for configuring the device floor location, 
 * shortcut call preferences, and device network status.
 */
Rectangle {
    id: configRoot

    color: "#f8f9fa"

    // Main content
    Column {
        anchors {
            top: parent.top
            horizontalCenter: parent.horizontalCenter
            topMargin: 30
        }

        width: parent.width * 0.8

        spacing: 45

        // Floor Configuration Section
        Column {
            id: floorSelections

            width: parent.width

            anchors.horizontalCenter: parent.horizontalCenter
            
            spacing: 16

            // In which floor the device is
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Floor"

                width: parent.width * 0.8
                
                model: uiHandler.allFloors
                textRole: "name"
                valueRole: "floorId"
                selectedValue: uiHandler.floorIdx
                
                onUserSelected: (val) => uiHandler.setMainFloor(val)
            }

            // Pref 1
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Pref 1"

                width: parent.width * 0.8
                
                model: uiHandler.pref1Model
                textRole: "name"
                valueRole: "floorId"
                selectedValue: uiHandler.fastCallIdx1
                
                onUserSelected: (val) => uiHandler.setPrefFloor(1, val)
            }

            // Pref 2
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Pref 2"

                width: parent.width * 0.8
                
                model: uiHandler.pref2Model
                textRole: "name"
                valueRole: "floorId"
                selectedValue: uiHandler.fastCallIdx2
                
                onUserSelected: (val) => uiHandler.setPrefFloor(2, val)
            }

            // Pref 3
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Pref 3"

                width: parent.width * 0.8
                
                model: uiHandler.pref3Model
                textRole: "name"
                valueRole: "floorId"
                selectedValue: uiHandler.fastCallIdx3
                
                onUserSelected: (val) => uiHandler.setPrefFloor(3, val)
            }
        }

        // Display IP-Address of device
        Column {
            anchors.horizontalCenter: parent.horizontalCenter
            
            spacing: 6

            Text {
                anchors.horizontalCenter: parent.horizontalCenter

                text: "System IP-Address"

                font {
                    pixelSize: 11
                    bold: true
                    letterSpacing: 1.5
                    family: "Inter"
                }

                color: "#6b7280"
            }

            Text {
                anchors.horizontalCenter: parent.horizontalCenter

                text: networkHandler.ipAddress

                font {
                    pixelSize: 20
                    weight: Font.DemiBold
                    family: "Inter"
                }

                color: "#1f2937"
            }
        }
    }
}