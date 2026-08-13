import QtQuick 2.15
import QtQuick.Controls 2.15
import UiBridge
import "../../components"

/**
 * Display-Settings Screen View
 * 
 * Provides UI controls for configuring the devices display brightness and screen timeout duration.
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

        // Brightness Slider
        Slider {
            label: "Screen-Brightness"
            val: SettingsBridge.brightness
            unit: "%"
            min: 10
            max: 100
            onMoved: (v) => SettingsBridge.brightness = v
        }

        // Display-Standby Slider
        Slider {
            label: "Display-Standby"
            val: SettingsBridge.displayTime
            unit: "sek"
            min: 10
            max: 300
            onMoved: (v) => SettingsBridge.displayTime = v
        }
    }
}