import QtQuick
import QtQuick.Shapes

Item {
    id: root

    property bool running: true
    property color color: "#79baff" // Modern vibrant blue
    property color trackColor: Qt.alpha(color, 0.15) // Automatically matches color with transparency
    property int lineWidth: 4
    property int spinDuration: 2000

    implicitWidth: 48
    implicitHeight: 48

    readonly property real radius: Math.max(0, Math.min(width, height) / 2 - lineWidth / 2)

    Item {
        id: container
        anchors.fill: parent
        opacity: root.running ? 1.0 : 0.0

        Behavior on opacity {
            NumberAnimation { duration: 250; easing.type: Easing.OutCubic }
        }

        Shape {
            id: shape
            anchors.fill: parent
            preferredRendererType: Shape.CurveRenderer

            // Static background track ring
            ShapePath {
                strokeColor: root.trackColor
                strokeWidth: root.lineWidth
                fillColor: "transparent"
                capStyle: ShapePath.RoundCap

                PathAngleArc {
                    centerX: shape.width / 2
                    centerY: shape.height / 2
                    radiusX: root.radius
                    radiusY: root.radius
                    startAngle: 0
                    sweepAngle: 360
                }
            }

            // Modern animated active progress arc
            ShapePath {
                id: activeArc
                strokeColor: root.color
                strokeWidth: root.lineWidth
                fillColor: "transparent"
                capStyle: ShapePath.RoundCap

                property real startAngle: 0
                property real sweepAngle: 30

                PathAngleArc {
                    centerX: shape.width / 2
                    centerY: shape.height / 2
                    radiusX: root.radius
                    radiusY: root.radius
                    startAngle: activeArc.startAngle
                    sweepAngle: activeArc.sweepAngle
                }
            }
        }

        // Continuous rotation of the entire spinner
        RotationAnimation on rotation {
            target: shape
            from: 0
            to: 360
            duration: root.spinDuration
            loops: Animation.Infinite
            running: root.running
        }

        // Fluid expansion and contraction of the arc
        SequentialAnimation {
            running: root.running
            loops: Animation.Infinite

            ParallelAnimation {
                NumberAnimation {
                    target: activeArc
                    property: "startAngle"
                    from: 0
                    to: 720
                    duration: root.spinDuration * 1.5
                    easing.type: Easing.InOutCubic
                }
                SequentialAnimation {
                    NumberAnimation {
                        target: activeArc
                        property: "sweepAngle"
                        from: 20
                        to: 270
                        duration: root.spinDuration * 0.75
                        easing.type: Easing.InOutCubic
                    }
                    NumberAnimation {
                        target: activeArc
                        property: "sweepAngle"
                        from: 270
                        to: 20
                        duration: root.spinDuration * 0.75
                        easing.type: Easing.InOutCubic
                    }
                }
            }
        }
    }
}
