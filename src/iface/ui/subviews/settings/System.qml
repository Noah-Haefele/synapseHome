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
            topMargin: 80
        }

        width: parent.width * 0.8

        spacing: 55

        // Selection of floors
        Column {
            id: floorSelections

            anchors.horizontalCenter: parent.horizontalCenter
            
            spacing: 15

            // In which floor the device is
            Dropdown {
                width: 400
                height: 56

                anchors.horizontalCenter: parent.horizontalCenter
                
                model: Floors {}
                textRole: "name"

                currentIndex: uiHandler.floorIdx

                onCurrentIndexChanged: {
                    if (uiHandler.floorIdx !== currentIndex) {
                        uiHandler.floorIdx = currentIndex
                    }
                }
            }

            // Floor call shortcut 1 in control grid
            Dropdown {
                width: 400
                height: 56

                anchors.horizontalCenter: parent.horizontalCenter
                
                model: Floors {}
                textRole: "name"
            }

            // Floor call shortcut 2 in control grid
            Dropdown {
                width: 400
                height: 56

                anchors.horizontalCenter: parent.horizontalCenter
                
                model: Floors {}
                textRole: "name"
            }

            // Floor call shortcut 3 in control grid
            Dropdown {
                width: 400
                height: 56

                anchors.horizontalCenter: parent.horizontalCenter
                
                model: Floors {}
                textRole: "name"
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