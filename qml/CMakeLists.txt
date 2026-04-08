cmake_minimum_required(VERSION 3.16)
project(sysmonitor)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_AUTOMOC ON)

find_package(Qt6 REQUIRED COMPONENTS Core Gui Qml Quick DBus)

add_executable(app main.cpp)

target_link_libraries(app
    Qt6::Core
    Qt6::Gui
    Qt6::Qml
    Qt6::Quick
    Qt6::DBus
)
