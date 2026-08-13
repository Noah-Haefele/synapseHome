import QtQuick 2.15
import QtQuick.Controls 2.15
import UiBridge
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
            
            model: SettingsBridge.outputModel
            textRole: "name"
            valueRole: "id"
            selectedValue: SettingsBridge.outputDevice
            
            onUserSelected: (val) => SettingsBridge.setSink(val)

            onDropdownOpened: {
                if (visible) {
                    SettingsBridge.onAudioDropdownOpened()
                }
            }
        }

        // Pref 1
        Dropdown {
            anchors.horizontalCenter: parent.horizontalCenter
            label: "Input"

            width: parent.width * 0.8
            
            model: SettingsBridge.inputModel
            textRole: "name"
            valueRole: "id"
            selectedValue: SettingsBridge.inputDevice
            
            onUserSelected: (val) => SettingsBridge.setSource(val)

            onDropdownOpened: {
                if (visible) {
                    SettingsBridge.onAudioDropdownOpened()
                }
            }
        }
    }
}