#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <memory>
#include <grpcpp/grpcpp.h>

#include "grpc_client.hpp"
#include "settings_bridge.hpp"


int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    auto channel = grpc::CreateChannel("localhost:50051", grpc::InsecureChannelCredentials());
    auto grpcClient = std::make_shared<Client>(channel, "settings_db");

    // create instance and bound to app
    auto settingsBridge = new SettingsBridge(grpcClient, &app);

    // c++ keeps memory contorl
    QQmlEngine::setObjectOwnership(settingsBridge, QQmlEngine::CppOwnership);

    // qml singletone
    qmlRegisterSingletonInstance("UiBridge", 1, 0, "SettingsBridge", settingsBridge);

    QQmlApplicationEngine engine;
    engine.loadFromModule("UiBridge", "Main");

    return app.exec();
}
