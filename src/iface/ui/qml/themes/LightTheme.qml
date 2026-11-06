import QtQuick

QtObject {
    readonly property QtObject colors: QtObject {
        readonly property color primaryBackground: "white"
        readonly property color secondaryBackground: "whitesmoke"
        readonly property color tertiaryBackground: "#e8e8e8" // Darker light-gray
        readonly property color text: "black"
        readonly property color primaryAccent: "lightseagreen"
        readonly property color secondaryAccent: "lightgrey"
    }
}
