import QtQuick 2.15
import QtQuick.Controls 2.15
import "../../components"

/**
 * Audio-Settings Screen View
 * 
 * Provides UI controls for configuring the devices audio input and output
 * as well as setting the volume and checking the volume
 */
Rectangle {
    id: root

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

        // In which floor the device is
        Dropdown {
            anchors.horizontalCenter: parent.horizontalCenter
            label: "Output"

            width: parent.width * 0.8
            
            model: uiHandler.outputModel
            textRole: "name"
            valueRole: "name"
            //selectedValue: uiHandler.floorIdx
            
            onUserSelected: (val) => uiHandler.setInputDevice(val)

            onDropdownOpened: {
                if (visible) {
                    uiHandler.onAudioDropdownOpened()
                }
            }
        }

        // Pref 1
        Dropdown {
            anchors.horizontalCenter: parent.horizontalCenter
            label: "Input"

            width: parent.width * 0.8
            
            model: uiHandler.inputModel
            textRole: "name"
            valueRole: "name"
            //selectedValue: uiHandler.fastCallIdx1
            
            onUserSelected: (val) => uiHandler.setOutputDevice(val)

            onDropdownOpened: {
                if (visible) {
                    uiHandler.onAudioDropdownOpened()
                }
            }
        }
    }
}