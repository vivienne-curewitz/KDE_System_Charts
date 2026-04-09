import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import "./components"

Window {
    visible: true
    width: 2560
    height: 1440
    title: "CPU / Memory Monitor"
    color: "#BC323232"
    //flags: Qt.FramelessWindowHint
    GridLayout {
        id: grid
        columns: 2
        //Layout.alignment: Qt.AlignTop
        Layout.fillWidth: true
        anchors.fill: parent
        Rectangle {
            id: barrect
            width: parent.width / 2
            height: parent.height
            color: "transparent"
            Layout.fillHeight: true
            Layout.fillWidth: true
            border.color: "white"
            border.width: 1
            radius: 1
            
            
            CpuBarChart {
                id: cpuBarChart
            }
        }
        Rectangle {
            id: linerect
            width: parent.width / 2
            height: parent.height
            color: "transparent"
            Layout.fillHeight: true
            Layout.fillWidth: true
            border.color: "white"
            border.width: 1
            radius: 1
            
            
            CpuLineChart {
                id: cpuLineChart
            }
        } 
        Rectangle {
            id: memlinerect
            width: parent.width / 2
            height: parent.height
            color: "transparent"
            border.color: "white"
            border.width: 1
            radius: 1
            
            
            CpuLineChart {
                id: fakecpuLineChart
            }
        }
    }
    
}
