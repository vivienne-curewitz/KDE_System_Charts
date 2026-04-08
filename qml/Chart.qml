import QtQuick
import QtQuick.Window
import org.kde.quickcharts 1.0 as Charts

Window {
    visible: true
    width: 400
    height: 300
    title: "CPU / Memory Monitor"

    property real minValue: 0
    property real maxValue: 100

    Charts.ArraySource {
        id: systemSource
        array: sysInfo ? sysInfo.system.map(v => Math.min(Math.max(v, minValue), maxValue)) : []
        function scaledArray() {
            var arr = sysInfo ? sysInfo.system.map(v => Math.min(Math.max(v, minValue), maxValue)) : []
            arr.push(minValue)
            arr.push(maxValue/2)
            return arr
        }
    }

    Charts.ArraySource {
        id: userSource
        array: sysInfo ? sysInfo.user.map(v => Math.min(Math.max(v, minValue), maxValue)) : []
        function scaledArray() {
            var arr = sysInfo ? sysInfo.user.map(v => Math.min(Math.max(v, minValue), maxValue)) : []
            arr.push(minValue)
            arr.push(maxValue/2)
            return arr
        }
    }

    Connections {
        target: sysInfo
        onStatsChanged: {
            systemSource.array = systemSource.scaledArray()
            userSource.array = userSource.scaledArray()
        }
    }

    Charts.BarChart {
        anchors.fill: parent
        anchors.margins: 20
        orientation: Qt.Horizontal
        stacked: true
        valueSources: [
            Charts.ArraySource { array: systemSource.scaledArray() },
            Charts.ArraySource { array: userSource.scaledArray() }
        ]
        colorSource: Charts.ArraySource {
            array: ["#FFA500", "#F400FF"]
        }
    }
}
