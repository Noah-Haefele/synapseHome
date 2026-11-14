#pragma once

#include <QObject>
#include <QtQml/qqmlregistration.h>

class GuiInstaller : public QObject
{
    Q_OBJECT

    Q_PROPERTY(
        int currentInstallationPage
        READ currentInstallationPage
        NOTIFY currentInstallationPageChanged
    )

    Q_PROPERTY(
        bool isGuiDevice
        READ isGuiDevice
        NOTIFY deviceTypeChanged
    )

public:
    // Device type selected in the installer
    enum DeviceType {
        GuiDevice,
        DoorDevice,
    };

    // Services selected in the installer
    enum Service {
        None,
        Backend,
        BackendGui,
    };

    explicit GuiInstaller(QObject *parent = nullptr);

    Q_ENUM(DeviceType);
    Q_ENUM(Service);

    Q_INVOKABLE void nextInstallationPage();
    Q_INVOKABLE void installDevice(DeviceType deviceType);
    Q_INVOKABLE void service(Service service);

    int currentInstallationPage() const;
    bool isGuiDevice() const;

private:
    int m_currentInstallationPage = 1;
    DeviceType m_deviceType;
    Service m_service;

signals:
    void currentInstallationPageChanged();
    void deviceTypeChanged();
};
