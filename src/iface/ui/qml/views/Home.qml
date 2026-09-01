import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ControlGridBridge
import CallBridge
import "../subviews"

Item {
    id: root

    RowLayout {
        anchors.fill: parent
        spacing: 0

        // left panel
        Rectangle {
            Layout.fillHeight: true
            Layout.preferredWidth: parent.width * 0.5
            color: "white"
        }

        // center
        Rectangle {
            Layout.fillHeight: true
            width: 2
            color: "black"
        }

        // right panel
        ControlGrid {
            Layout.fillHeight: true
            Layout.fillWidth: true

            onSingleClicked:  (type, value) => {
                if (type === "floor") {
                    const device_id = ControlGridBridge.getPrefCallId(value)

                    if (device_id >= 0) {
                        CallBridge.initiateCall(device_id)
                    }
                } else{
                    switch (value) {
                        case "settings":
                            stackView.push("Settings.qml")
                            break
                    }
                }
            }
        }
    }
}
