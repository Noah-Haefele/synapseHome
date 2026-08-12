cmake_minimum_required(VERSION 3.16)

# sHUiBridge = synapseHomeUiBridge
project(sHUiBridge VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(PROJECT_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/../../../)

find_package(Qt6 6.5 COMPONENTS REQUIRED
    Quick
    Qml
    Gui
)

qt_standard_project_setup(REQUIRES 6.5)

qt_add_executable(sHUiBridge
    main.cpp
)

set(QML_FILES
    qml/Main.qml

    qml/views/Home.qml
    qml/views/Settings.qml
    qml/views/call/Accepted.qml
    qml/views/call/Incoming.qml
    qml/views/call/Outgoing.qml

    qml/subviews/ControlGrid.qml
    qml/subviews/settings/Audio.qml
    qml/subviews/settings/Display.qml
    qml/subviews/settings/System.qml

    qml/models/SettingsOptions.qml

    qml/components/ButtonIcon.qml
    qml/components/CallIcon.qml
    qml/components/Dropdown.qml
    qml/components/NavBar.qml
    qml/components/SimpleButton.qml
    qml/components/Slider.qml
)

set(SRC_FILES
    src/settings_bridge.hpp
    src/settings_bridge.cpp
)

set(ICON_FILES
    ${PROJECT_ROOT}/assets/icons/button/bulb_off.svg
    ${PROJECT_ROOT}/assets/icons/button/bulb_on.svg
    ${PROJECT_ROOT}/assets/icons/button/call.svg
    ${PROJECT_ROOT}/assets/icons/button/muteBell_off.svg
    ${PROJECT_ROOT}/assets/icons/button/muteBell_on.svg
    ${PROJECT_ROOT}/assets/icons/button/settings.svg

    ${PROJECT_ROOT}/assets/icons/call/call_off.svg
    ${PROJECT_ROOT}/assets/icons/call/call_on.svg
    ${PROJECT_ROOT}/assets/icons/call/muteMic_off.svg
    ${PROJECT_ROOT}/assets/icons/call/muteMic_on.svg
    ${PROJECT_ROOT}/assets/icons/call/openDoor.svg
)

foreach(ICON ${ICON_FILES})
    file(RELATIVE_PATH ICON_RELATIVE
        "${PROJECT_ROOT}"
        "${ICON}"
    )

    set_source_files_properties(
        "${ICON}"
        PROPERTIES
        QT_RESOURCE_ALIAS "${ICON_RELATIVE}"
    )
endforeach()

qt_add_qml_module(sHUiBridge
    URI UiBridge
    VERSION 1.0.0

    QML_FILES ${QML_FILES}
    SOURCES ${SRC_FILES}
    RESOURCES ${ICON_FILES}
)

target_include_directories(sHUiBridge PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

target_link_libraries(sHUiBridge PRIVATE
    Qt6::Quick
    Qt6::Qml
    Qt6::Gui
)