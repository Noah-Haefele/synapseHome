cmake_minimum_required(VERSION 4.3.4)

# sHUiBridge = synapseHomeUiBridge
project(sHUiBridge VERSION 1.0.0 LANGUAGES CXX)

include(FetchContent)

# Prevent Protobuf and utf8_range from exporting install targets
set(protobuf_INSTALL OFF)
set(utf8_range_ENABLE_INSTALL OFF)

FetchContent_Declare(
  gRPC
  GIT_REPOSITORY https://github.com/grpc/grpc
  GIT_TAG        v1.78.1
)
set(FETCHCONTENT_QUIET OFF)
FetchContent_MakeAvailable(gRPC)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(PROJECT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../../../")

# --- Grpc setup ---
# Proto file
get_filename_component(hw_proto
    "${PROJECT_ROOT}/proto/settings_api.proto"
    ABSOLUTE
)
get_filename_component(hw_proto_path
    "${hw_proto}"
    PATH
)

# Generated sources
set(hw_proto_srcs "${CMAKE_CURRENT_BINARY_DIR}/settings_api.pb.cc")
set(hw_proto_hdrs "${CMAKE_CURRENT_BINARY_DIR}/settings_api.pb.h")
set(hw_grpc_srcs "${CMAKE_CURRENT_BINARY_DIR}/settings_api.grpc.pb.cc")
set(hw_grpc_hdrs "${CMAKE_CURRENT_BINARY_DIR}/settings_api.grpc.pb.h")

set(_PROTOBUF_PROTOC $<TARGET_FILE:protoc>)
set(_GRPC_CPP_PLUGIN_EXECUTABLE $<TARGET_FILE:grpc_cpp_plugin>)

add_custom_command(
    OUTPUT
        "${hw_proto_srcs}"
        "${hw_proto_hdrs}"
        "${hw_grpc_srcs}"
        "${hw_grpc_hdrs}"

    COMMAND ${_PROTOBUF_PROTOC}
    ARGS
        --grpc_out "${CMAKE_CURRENT_BINARY_DIR}"
        --cpp_out "${CMAKE_CURRENT_BINARY_DIR}"
        -I "${hw_proto_path}"
        -I "${grpc_SOURCE_DIR}/third_party/protobuf/src"
        --plugin=protoc-gen-grpc="${_GRPC_CPP_PLUGIN_EXECUTABLE}"
        "${hw_proto}"

    DEPENDS "${hw_proto}"
)

find_package(Qt6 6.5 COMPONENTS REQUIRED
    Quick
    Qml
    Gui
)

qt_standard_project_setup(REQUIRES 6.5)

qt_add_executable(sHUiBridge
    main.cpp
    ${hw_proto_srcs}
    ${hw_proto_hdrs}
    ${hw_grpc_srcs}
    ${hw_grpc_hdrs}
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
    api/grpc_client.hpp
    api/grpc_client.cpp
)

set(ICON_FILES
    "${PROJECT_ROOT}/assets/icons/button/bulb_off.svg"
    "${PROJECT_ROOT}/assets/icons/button/bulb_on.svg"
    "${PROJECT_ROOT}/assets/icons/button/call.svg"
    "${PROJECT_ROOT}/assets/icons/button/muteBell_off.svg"
    "${PROJECT_ROOT}/assets/icons/button/muteBell_on.svg"
    "${PROJECT_ROOT}/assets/icons/button/settings.svg"

    "${PROJECT_ROOT}/assets/icons/call/call_off.svg"
    "${PROJECT_ROOT}/assets/icons/call/call_on.svg"
    "${PROJECT_ROOT}/assets/icons/call/muteMic_off.svg"
    "${PROJECT_ROOT}/assets/icons/call/muteMic_on.svg"
    "${PROJECT_ROOT}/assets/icons/call/openDoor.svg"
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
    "${CMAKE_CURRENT_SOURCE_DIR}/src"
    "${CMAKE_CURRENT_SOURCE_DIR}/api"
    "${CMAKE_CURRENT_BINARY_DIR}"
)

target_link_libraries(sHUiBridge PRIVATE
    grpc++
    Qt6::Quick
    Qt6::Qml
    Qt6::Gui
)