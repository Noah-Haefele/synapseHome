cmake_minimum_required(VERSION 3.31.6)

project(synapseHomeInstaller VERSION 1.0.0 LANGUAGES CXX)

include(FetchContent)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

get_filename_component(PROJECT_ROOT
    "${CMAKE_CURRENT_SOURCE_DIR}/../"
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

set(THEME_FILES
    "${PROJECT_ROOT}/assets/themes/Theme.qml"
    "${PROJECT_ROOT}/assets/themes/LightTheme.qml"
)

set(QML_FILES
    qml/Main.qml

    ${THEME_FILES}

    qml/components/WizardStep.qml
    qml/components/ModernButton.qml
    qml/components/RadioButton.qml

    qml/subviews/Welcome.qml
    qml/subviews/DeviceSelection.qml
    qml/subviews/SystemServices.qml
    qml/subviews/Finish.qml
)

foreach(THEME ${THEME_FILES})
    file(RELATIVE_PATH THEME_RELATIVE
        "${PROJECT_ROOT}"
        "${THEME}"
    )

    set_source_files_properties(
        "${THEME}"
        PROPERTIES
        QT_RESOURCE_ALIAS "${THEME_RELATIVE}"
    )
endforeach()

set_source_files_properties(
    "${PROJECT_ROOT}/assets/themes/Theme.qml"
    PROPERTIES
    QT_QML_SINGLETON_TYPE TRUE
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
