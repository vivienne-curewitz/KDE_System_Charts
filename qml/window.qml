import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import "./components"

Window {
    visible: true
    width: 1280
    height: 1440
    title: "CPU / Memory Monitor"
    color: "#BC323232"
    //flags: Qt.FramelessWindowHint
    Rectangle {
        id: barrect
        width: parent.width / 2
        height: parent.height / 2
        color: "transparent"
        border.color: "white"
        border.width: 1
        radius: 1
        
        CpuBarChart {
            anchors.fill: barrect
            id: cpuBarChart
        }
    }
    Rectangle {
        anchors.top: parent.top
        anchors.left: barrect.right
        id: linerect
        width: parent.width / 2
        height: parent.height / 2
        color: "transparent"
        border.color: "white"
        border.width: 1
        radius: 1
        
        
        CpuLineChart {
            anchors.fill: linerect
            id: cpuLineChart
        }
    } 
    Rectangle {
        anchors.top: barrect.bottom
        id: memlinerect
        width: parent.width / 2
        height: parent.height / 2
        color: "transparent"
        border.color: "white"
        border.width: 1
        radius: 1
        
        
        CpuLineChart {
            anchors.fill: parent
            id: fakecpuLineChart
        }
    }
    
}
