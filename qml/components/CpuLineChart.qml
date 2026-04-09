import QtQuick 2.0
import org.kde.quickcharts 1.0 as Charts
import QtQml.Models

Item {
    id: root
    visible: true
    width: 800
    height: 600

    property real minValue: 0
    property real maxValue: 100
    Instantiator {
        id: historyInstantiator
        // Use the flat array length to define how many lines to draw
        model: sysInfo.user.length 

        delegate: Charts.ArraySource {
            array: {
                // 🔹 THE TRICK: By referencing 'sysInfo.user' here, 
                // QML knows to re-run this entire block whenever statsChanged is emitted.
                var triggerUpdate = sysInfo.user; 
                
                // Ask C++ for exactly the 1D array we need for this specific line
                return sysInfo.getUserHistory(index);
            }
        }

        onObjectAdded: lineChart.valueSources = getSources()
        onObjectRemoved: lineChart.valueSources = getSources()

        function getSources() {
            var s = [];
            for (var i = 0; i < count; i++) {
                s.push(objectAt(i));
            }
            return s;
        }
    }
    Charts.LineChart {
        id: lineChart
        anchors.fill: parent
        anchors.margins: 20
        valueSources: currentSystemSource
        // Fixed Y-axis for stability
        yRange.automatic: false

        // valueSources is populated by the updateChartSources() function above
        
        colorSource: Charts.ArraySource {
            // You can provide a long list of colors or use a function to generate them
            array: ["#FFA500", "#F400FF", "#00FFCC", "#FF0000", "#FFFF00"]
        }
    }
}
