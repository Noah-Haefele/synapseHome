#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <grpcpp/grpcpp.h>
#include <QThread>

#include "grpc_client.hpp"
#include "settings_bridge.hpp"
#include "control_grid_bridge.hpp"
#include "grpc_signals_client.hpp"


int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    auto channel = grpc::CreateChannel("localhost:50051", grpc::InsecureChannelCredentials());
    auto grpcClient = std::make_shared<Client>(channel, "settings_db");

    // create instance and bound to app
    auto settingsBridge = new SettingsBridge(grpcClient, &app);
    auto controlGridBridge = new ControlGridBridge(grpcClient, &app);

    auto signalClient = new SignalClient(channel);

    QObject::connect(
        signalClient,
        &SignalClient::callIconChanged,
        controlGridBridge,
        &ControlGridBridge::iconChanged
    );

    // c++ keeps memory contorl
    QQmlEngine::setObjectOwnership(settingsBridge, QQmlEngine::CppOwnership);
    QQmlEngine::setObjectOwnership(controlGridBridge, QQmlEngine::CppOwnership);

    // qml singletone
    qmlRegisterSingletonInstance("SettingsBridge", 1, 0, "SettingsBridge", settingsBridge);
    qmlRegisterSingletonInstance("ControlGridBridge", 1, 0, "ControlGridBridge", controlGridBridge);

    QQmlApplicationEngine engine;
    engine.loadFromModule("UiBridge", "Main");

    // --- Create thread for signal client ---

    auto signalThread = new QThread(&app);

    signalClient->moveToThread(signalThread);

    QObject::connect(
        signalThread,
        &QThread::started,
        signalClient,
        &SignalClient::subscribe
    );

    signalThread->start();

    // Safe thread stop
    QObject::connect(
        &app,
        &QCoreApplication::aboutToQuit,
        [&]() {
            signalClient->stop();

            signalThread->quit();
            signalThread->wait();
        }
    );

    return app.exec();
}
