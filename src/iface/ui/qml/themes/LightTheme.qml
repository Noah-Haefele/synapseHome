import QtQuick

QtObject {
    readonly property QtObject colors: QtObject {
        readonly property color primaryBackground: "white"
        readonly property color secondaryBackground: "whitesmoke"
        readonly property color tertiaryBackground: "#e8e8e8" // Darker light-grey
        readonly property color gridBackground: "white"
        readonly property color text: "black"
        readonly property color primaryAccent: "lightseagreen"
        readonly property color secondaryAccent: "lightgrey"
        readonly property color blackSeperator: "#515151" // Lighter black
        readonly property color greySeperator: "#dbdbdd" // Lighter grey
    }
}
