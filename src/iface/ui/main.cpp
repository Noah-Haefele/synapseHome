#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <grpcpp/grpcpp.h>
#include <thread>

#include "api/grpc_client.hpp"
#include "api/grpc_call_client.hpp"
#include "src/control_grid_bridge.hpp"
#include "src/settings_bridge.hpp"
#include "src/call_bridge.hpp"


int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    auto channel = grpc::CreateChannel("localhost:50051", grpc::InsecureChannelCredentials());
    auto grpcClient = std::make_shared<Client>(channel, "settings_db");
    auto grpcCallClient = new GrpcCallClient(channel, &app);
    // Start blocking subscribe method
    std::thread([grpcCallClient]() {
        grpcCallClient->subscribe();
    }).detach();

    // create instance and bound to app
    auto settingsBridge = new SettingsBridge(grpcClient, &app);
    auto controlGridBridge = new ControlGridBridge(grpcClient, &app);
    auto callBridge = new CallBridge(&app);

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

    // c++ keeps memory contorl
    QQmlEngine::setObjectOwnership(settingsBridge, QQmlEngine::CppOwnership);
    QQmlEngine::setObjectOwnership(controlGridBridge, QQmlEngine::CppOwnership);

    // qml singletone
    qmlRegisterSingletonInstance("SettingsBridge", 1, 0, "SettingsBridge", settingsBridge);
    qmlRegisterSingletonInstance("ControlGridBridge", 1, 0, "ControlGridBridge", controlGridBridge);
    qmlRegisterSingletonInstance("CallBridge", 1, 0, "CallBridge", callBridge);

    QQmlApplicationEngine engine;
    engine.loadFromModule("UiBridge", "Main");

    return app.exec();
}
