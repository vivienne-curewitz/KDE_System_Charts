import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import org.kde.quickcharts 1.0 as Charts

Window {
    width: 400
    height: 300
    visible: true

    Charts.ArraySource {
        id: systemSource
        array: sysInfo.system
    }

    Charts.ArraySource {
        id: userSource
        array: sysInfo.user
    }

    Connections {
        target: sysInfo
        function onStatsChanged() {
            systemSource.array = sysInfo.system
            userSource.array = sysInfo.user
        }
    }

    Charts.BarChart {
        anchors.fill: parent
        anchors.margins: 20
    
        orientation: Qt.Horizontal
        stacked: true

        valueSources: [systemSource, userSource]

        colorSource: Charts.ArraySource {
            array: ["#FFA500", "#F400FF"]
        }
    }
}
