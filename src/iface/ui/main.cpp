#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <grpcpp/grpcpp.h>
#include <memory>
#include <thread>

#include "api/grpc_client.hpp"
#include "api/grpc_call_client.hpp"
#include "src/control_grid_bridge.hpp"
#include "src/settings_bridge.hpp"
#include "src/call_bridge.hpp"
#include "src/grpc_connection_monitor.hpp"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    grpc::ChannelArguments args;
    args.SetInt(GRPC_ARG_INITIAL_RECONNECT_BACKOFF_MS, 1000);
    args.SetInt(GRPC_ARG_MAX_RECONNECT_BACKOFF_MS, 1000);
    args.SetInt(GRPC_ARG_MIN_RECONNECT_BACKOFF_MS, 1000);

    auto channel = grpc::CreateCustomChannel(
        "localhost:50051",
        grpc::InsecureChannelCredentials(),
        args
    );
    auto grpcClient = std::make_shared<Client>(channel, "settings_db");
    auto grpcCallClient = new GrpcCallClient(channel, &app);
    // Start blocking subscribe method
    std::thread([grpcCallClient]() {
        grpcCallClient->subscribe();
    }).detach();

    // Start the gRPC connection monitor
    auto grpcConnectionMonitor = new GrpcConnectionMonitor(channel, &app);

    // create instance and bound to app
    auto settingsBridge = new SettingsBridge(grpcClient, &app);
    auto controlGridBridge = new ControlGridBridge(grpcClient, &app);
    auto callBridge = new CallBridge(grpcCallClient, &app);
    
    // Connect signals
    // Connect pref_call_icon_changed signals between settings_bridge and control_grid_bridge
    QObject::connect(
        settingsBridge,
        &SettingsBridge::pref_call_icon_changed,
        controlGridBridge,
        &ControlGridBridge::pref_call_icon_changed
    );

    QObject::connect(
        grpcCallClient,
        &GrpcCallClient::callStateChanged,
        callBridge,
        &CallBridge::callStateChanged
    );

    QObject::connect(
        grpcConnectionMonitor,
        &GrpcConnectionMonitor::reconnected,
        settingsBridge,
        &SettingsBridge::handleReconnect
    );

    QObject::connect(
        grpcConnectionMonitor,
        &GrpcConnectionMonitor::reconnected,
        controlGridBridge,
        &ControlGridBridge::handleReconnect
    );

    // c++ keeps memory contorl
    QQmlEngine::setObjectOwnership(settingsBridge, QQmlEngine::CppOwnership);
    QQmlEngine::setObjectOwnership(controlGridBridge, QQmlEngine::CppOwnership);

    // qml singletone
    qmlRegisterSingletonInstance("SettingsBridge", 1, 0, "SettingsBridge", settingsBridge);
    qmlRegisterSingletonInstance("ControlGridBridge", 1, 0, "ControlGridBridge", controlGridBridge);
    qmlRegisterSingletonInstance("CallBridge", 1, 0, "CallBridge", callBridge);
    qmlRegisterSingletonInstance("ConnectionMonitor", 1, 0, "ConnectionMonitor", grpcConnectionMonitor);

    QQmlApplicationEngine engine;
    engine.loadFromModule("UiBridge", "Main");

    grpcConnectionMonitor->start();

    return app.exec();
}
