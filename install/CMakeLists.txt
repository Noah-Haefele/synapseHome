cmake_minimum_required(VERSION 3.31.6)

project(synapseHomeInstaller VERSION 1.0.0 LANGUAGES CXX)

include(FetchContent)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

get_filename_component(PROJECT_ROOT
    "${CMAKE_CURRENT_SOURCE_DIR}/../../../"
    ABSOLUTE
)

find_package(Qt6 6.5 COMPONENTS REQUIRED
    Quick
    Qml
    Gui
)

qt_standard_project_setup(REQUIRES 6.5)

qt_add_executable(synapseHomeInstaller
    main.cpp
)

set(QML_FILES
    qml/Main.qml
    qml/Start.qml
)

qt_add_qml_module(synapseHomeInstaller
    URI sHInstaller
    VERSION 1.0.0

    QML_FILES ${QML_FILES}
)

target_include_directories(synapseHomeInstaller PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/src"
    "${CMAKE_CURRENT_BINARY_DIR}"
)

target_link_libraries(synapseHomeInstaller PRIVATE
    Qt6::Quick
    Qt6::Qml
    Qt6::Gui
)
