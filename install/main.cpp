#include <QGuiApplication>
#include <QQmlApplicationEngine>

#include "src/gui_installer.hpp"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    auto guiInstaller = new GuiInstaller(&app);

    // qml singleton
    qmlRegisterSingletonInstance("GuiInstaller", 1, 0, "GuiInstaller", guiInstaller);

    QQmlApplicationEngine engine;
    engine.loadFromModule("sHInstaller", "Main");

    return app.exec();
}
