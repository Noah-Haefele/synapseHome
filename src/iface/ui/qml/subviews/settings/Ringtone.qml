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

    // Main content
    Column {
        anchors {
            top: parent.top
            horizontalCenter: parent.horizontalCenter
            topMargin: 30
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
