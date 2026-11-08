import QtQuick

QtObject {
    readonly property QtObject colors: QtObject {
        readonly property color primaryBackground: "white"
        readonly property color secondaryBackground: "whitesmoke"
        readonly property color tertiaryBackground: "#e8e8e8" // Darker light-grey
        readonly property color gridBackground: "white"
        // Different colors of text are just lighter blacks
        readonly property color text: "black"
        readonly property color secondaryText: "#444444"
        readonly property color tertiaryText: "#666666"
        readonly property color oppositeText: "white"
        readonly property color primaryAccent: "lightseagreen"
        readonly property color secondaryAccent: "lightgrey"
        readonly property color blackSeperator: "#515151" // Lighter black
        readonly property color greySeperator: "#dbdbdd" // Lighter grey
        readonly property color red: "red";
        readonly property color green: "lightseagreen";
        readonly property color blue: "#2563EB"; // = blue
    }
}
