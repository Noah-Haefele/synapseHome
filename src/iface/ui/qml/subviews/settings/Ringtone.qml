import QtQuick 2.15
import QtQuick.Controls 2.15
import SettingsBridge
import "../../components"

/**
 * Ringtone-Settings Screen View
 *
 * Provides UI controls for configuring the ringtone
 */
Rectangle {
    id: root

    color: "#f8f9fa"

    Component.onCompleted: {
        SettingsBridge.refresh_ringtones()
    }

    // Reload button positioned at the top-right corner
    SimpleButton {
        id: reloadButton
        anchors {
            top: parent.top
            right: parent.right
            topMargin: 9.5
            rightMargin: 9.5
        }

        // should align the control buttons in the navbar
        height: parent.height * 0.11 * 0.8
        width: height

        backgroundColor: root.color
        textColor: "black"
        radius: 8

        text: "\u21bb" // Reload / refresh unicode symbol
        pixelsize: height * 0.45

        // Reload ringtones
        // Required when user uploads ringtones and dont wants to restart the entire application
        onClicked: SettingsBridge.refresh_ringtones()
    }

    // Main content
    Column {
        anchors {
            top: parent.top
            horizontalCenter: parent.horizontalCenter
            topMargin: 80 // Offset to prevent overlapping with the reload button
        }

        width: parent.width * 0.8
        spacing: 45

        // Ringtone
        Dropdown {
            anchors.horizontalCenter: parent.horizontalCenter
            label: "Ringtone"

            width: parent.width * 0.8

            model: SettingsBridge.ringtone_model
            textRole: "formatted_name"
            valueRole: "id"
            selectedValue: SettingsBridge.ringtone_id

            onUserSelected: (val) => SettingsBridge.set_ringtone_id(val)
        }
    }
}
