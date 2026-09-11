import QtQuick 2.15
import QtQuick.Controls 2.15

/**
 * Custom Dropdown / ComboBox Component
 *
 * Wraps QtQuick ComboBox with automatic index synchronization
 * based on backend model keys (valueRole) and dynamic popup positioning.
 */

Column {
    id: root

    property alias model: combo.model
    property alias textRole: combo.textRole
    property alias count: combo.count
    property alias label: label.text

    // Model key used to extract the option's unique identifier/ID
    property alias valueRole: combo.valueRole

    // Active selection provided by backend state
    property var selectedValue: -1

    readonly property alias popupVisible: popup.visible

    // Emitted only on explicit user interaction
    signal userSelected(var value)
    // Emitted when popup opened
    signal dropdownOpened()

    width: 350
    spacing: 1

    /**
     * Synchronizes the internal ComboBox currentIndex with `selectedValue`.
     * Guards against uninitialized properties or empty models.
     */
    function syncIndex() {
        if (!combo || combo.count === 0 || combo.valueRole === "" || selectedValue === undefined) {
            return
        }
        var idx = combo.indexOfValue(selectedValue)
        if (idx !== -1) {
            combo.currentIndex = idx
        }
    }

    // Defer sync execution to avoid initial property binding race conditions
    onSelectedValueChanged: Qt.callLater(syncIndex)
    onValueRoleChanged: Qt.callLater(syncIndex)
    Component.onCompleted: Qt.callLater(syncIndex)

    Text {
        id: label

        visible: text.length > 0
        text: ""

        font.pixelSize: 11

        color: "#6b7280"
    }

    ComboBox {
        id: combo

        height: 60
        width: root.width

        font.pixelSize: 24
        font.family: "DejaVu Sans, sans-serif"

        // Re-synchronize when model items update dynamically
        onModelChanged: Qt.callLater(root.syncIndex)
        onCountChanged: Qt.callLater(root.syncIndex)

        onActivated: function(index) {
            var val = combo.valueAt(index)
            if (val !== undefined) {
                root.userSelected(val)
            }
        }

        delegate: ItemDelegate {
            width: combo.width
            height: 70

            // Safely resolve display text from Python dictionaries or primitive values
            text: {
                if (typeof modelData === "object" && modelData !== null) {
                    return combo.textRole !== "" && modelData[combo.textRole] !== undefined
                           ? modelData[combo.textRole]
                           : ""
                }
                return modelData !== undefined ? modelData : ""
            }

            contentItem: Text {
                text: parent.text
                color: highlighted ? "#2F4C6B" : "#333333"
                font.pixelSize: 22
                verticalAlignment: Text.AlignVCenter
                leftPadding: 20
            }

            background: Rectangle {
                color: highlighted ? "#f0f0f0" : "#ffffff"
            }
        }

        // Dropdown arrow indicator
        indicator: Text {
            x: combo.width - width - 20
            anchors.verticalCenter: parent.verticalCenter
            text: combo.popup.visible ? "▲" : "▼"
            color: "#999999"
            font.pixelSize: 18
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
        }

        background: Rectangle {
            color: "#ffffff"
            radius: 12
            border.color: "#e0e0e0"
            border.width: 1
        }

        // content
        contentItem: Text {
            text: combo.displayText

            color: "#333333"
            font: combo.font

            verticalAlignment: Text.AlignVCenter

            leftPadding: 20
            // prevents overlapping with arrow
            rightPadding: combo.indicator ? combo.indicator.width + 30 : 20

            elide: Text.ElideRight
        }

        popup: Popup {
            id: popup

            // Item height is 70px; maximum popup height cap is 400px
            property real expectedHeight: Math.min(combo.count * 70, 400)

            onVisibleChanged: {
                if (visible) {
                    root.dropdownOpened()
                }
            }

            // set height so qt knows the bounds
            height: expectedHeight
            width: combo.width

            // calculates if popup has to be opend upwards
            property bool opensUpwards: {
                if (!combo.Window.window) return false;

                // calculates lower edge of popup
                let globalBottom = combo.mapToItem(null, 0, combo.height).y;
                let spaceBelow = combo.Window.window.height - globalBottom;

                // if lower edge of popup smaller than actual popup height
                return spaceBelow < expectedHeight + 10;
            }

            // upwards: -height of popup - 10
            // downwards: height of popup + 10
            y: opensUpwards ? -expectedHeight - 15 : combo.height + 15

            // flip animation from right direction
            transformOrigin: opensUpwards ? Popup.Bottom : Popup.Top

            background: Rectangle {
                color: "#ffffff"
                radius: 12

                border.color: "#e0e0e0"
                border.width: 1
            }

            // available options
            contentItem: ListView {
                model: combo.delegateModel

                clip: true
                spacing: 0

                implicitHeight: Math.min(contentHeight, 400)
            }
        }
    }
}
