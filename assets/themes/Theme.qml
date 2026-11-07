pragma Singleton
import QtQuick

QtObject {
    id: root

    readonly property LightTheme lightTheme: LightTheme {}

    property QtObject currentTheme: lightTheme

    readonly property var colors: currentTheme.colors
}
