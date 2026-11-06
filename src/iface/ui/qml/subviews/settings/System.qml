import QtQuick 2.15
import QtQuick.Controls 2.15
import SettingsBridge
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

    color: Theme.colors.primaryBackground

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

            spacing: 30

            // Which available device (in devices.json) the device is
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Device Identification"

                width: parent.width * 0.8
                height: 60

                pixelsize: height * 0.25

                model: SettingsBridge.all_devices
                textRole: "device_name"
                valueRole: "device_id"
                selectedValue: SettingsBridge.location_id

                onUserSelected: (val) => SettingsBridge.set_location_id(val)
                
                labelColor: Theme.colors.text
                textColor: Theme.colors.text
                accentColor: Theme.colors.secondaryAccent
                backgroundColor: Theme.colors.primaryBackground
            }

            // Pref 1
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Primary Call Preference"

                width: parent.width * 0.8
                height: 60

                pixelsize: height * 0.25

                model: SettingsBridge.pref_model
                textRole: "device_name"
                valueRole: "device_id"
                selectedValue: SettingsBridge.pref1_call_id

                onUserSelected: (val) => SettingsBridge.set_pref_call_id(1, val)

                labelColor: Theme.colors.text
                textColor: Theme.colors.text
                accentColor: Theme.colors.secondaryAccent
                backgroundColor: Theme.colors.primaryBackground
            }

            // Pref 2
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Secondary Call Preference"

                width: parent.width * 0.8
                height: 60

                pixelsize: height * 0.25

                model: SettingsBridge.pref_model
                textRole: "device_name"
                valueRole: "device_id"
                selectedValue: SettingsBridge.pref2_call_id

                onUserSelected: (val) => SettingsBridge.set_pref_call_id(2, val)

                labelColor: Theme.colors.text
                textColor: Theme.colors.text
                accentColor: Theme.colors.secondaryAccent
                backgroundColor: Theme.colors.primaryBackground
            }

            // Pref 3
            Dropdown {
                anchors.horizontalCenter: parent.horizontalCenter
                label: "Tertiary Call Preference"

                width: parent.width * 0.8
                height: 60

                pixelsize: height * 0.25

                model: SettingsBridge.pref_model
                textRole: "device_name"
                valueRole: "device_id"
                selectedValue: SettingsBridge.pref3_call_id

                onUserSelected: (val) => SettingsBridge.set_pref_call_id(3, val)

                labelColor: Theme.colors.text
                textColor: Theme.colors.text
                accentColor: Theme.colors.secondaryAccent
                backgroundColor: Theme.colors.primaryBackground
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

                color: Theme.colors.text
            }

            Text {
                anchors.horizontalCenter: parent.horizontalCenter

                text: SettingsBridge.ip_address

                font {
                    pixelSize: 20
                    weight: Font.DemiBold
                    family: "Inter"
                }

                color: Theme.colors.text
            }
        }
    }
}
