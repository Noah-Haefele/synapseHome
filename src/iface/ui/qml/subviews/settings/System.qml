import QtQuick 2.15
import QtQuick.Controls 2.15
import UiBridge
import "../../components"
import "../../models"

/**
 * System-Settings Screen View
 * 
 * Provides UI controls for identifying the device, 
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

        // Device Configuration Section
        Column {
            id: deviceSelections

            width: parent.width

            anchors.horizontalCenter: parent.horizontalCenter
            
            spacing: 16

            // Which available device (in devices.json) the device is
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Device Identification"

                width: parent.width * 0.8
                
                model: SettingsBridge.all_devices
                textRole: "device_name"
                valueRole: "device_id"
                selectedValue: SettingsBridge.location_id
                
                onUserSelected: (val) => SettingsBridge.set_pref_call_id(val)
            }

            // Pref 1
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Pref 1"

                width: parent.width * 0.8
                
                model: SettingsBridge.pref_model
                textRole: "device_name"
                valueRole: "device_id"
                selectedValue: SettingsBridge.pref_call_id1
                
                onUserSelected: (val) => SettingsBridge.set_pref_call_id(1, val)
            }

            // Pref 2
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Pref 2"

                width: parent.width * 0.8
                
                model: SettingsBridge.pref_model
                textRole: "device_name"
                valueRole: "device_id"
                selectedValue: SettingsBridge.pref_call_id2
                
                onUserSelected: (val) => SettingsBridge.set_pref_call_id(2, val)
            }

            // Pref 3
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Pref 3"

                width: parent.width * 0.8
                
                model: SettingsBridge.pref_model
                textRole: "device_name"
                valueRole: "device_id"
                selectedValue: SettingsBridge.pref_call_id3
                
                onUserSelected: (val) => SettingsBridge.set_pref_call_id(3, val)
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