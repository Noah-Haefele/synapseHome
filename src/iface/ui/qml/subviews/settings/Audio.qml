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

        // Sink (audio output)
        Dropdown {
            anchors.horizontalCenter: parent.horizontalCenter
            label: "Output"

            width: parent.width * 0.8
            
            model: SettingsBridge.output_model
            textRole: "name"
            valueRole: "id"
            selectedValue: SettingsBridge.output_device
            
            onUserSelected: (val) => SettingsBridge.set_sink(val)

            onDropdownOpened: {
                if (visible) {
                    SettingsBridge.onAudioDropdownOpened()
                }
            }
        }

        // Source (audio input)
        Dropdown {
            anchors.horizontalCenter: parent.horizontalCenter
            label: "Input"

            width: parent.width * 0.8
            
            model: SettingsBridge.input_model
            textRole: "name"
            valueRole: "id"
            selectedValue: SettingsBridge.input_device
            
            onUserSelected: (val) => SettingsBridge.set_source(val)

            onDropdownOpened: {
                if (visible) {
                    SettingsBridge.onAudioDropdownOpened()
                }
            }
        }
    }
}