#include "gui_installer.hpp"
#include <iostream>

GuiInstaller::GuiInstaller(QObject *parent)
    : QObject(parent)
{
}

void GuiInstaller::nextInstallationPage()
{
    m_currentInstallationPage++;
    emit currentInstallationPageChanged();
}

void GuiInstaller::installDevice(GuiInstaller::DeviceType deviceType)
{
    if (m_deviceType != deviceType) {
        m_deviceType = deviceType;
        emit deviceTypeChanged();

        switch (deviceType) {
            case GuiDevice:
                std::cout << "GuiDevice" << "\n";
                break;
            case DoorDevice:
                std::cout << "DoorDevice" << "\n";
                break;
        }
    }
}

void GuiInstaller::service(GuiInstaller::Service service)
{
    if (m_service != service) {
        // Validate if the service matches the deviceType
        if (m_deviceType == DoorDevice && m_service == BackendGui) return;

        m_service = service;

        switch (service) {
            case None:
                std::cout << "No service" << "\n";
                break;
            case Backend:
                std::cout << "Backend service" << "\n";
                break;
            case BackendGui:
                std::cout << "Backend+Gui service" << "\n";
                break;
        }
    }
}

int GuiInstaller::currentInstallationPage() const
{
    return m_currentInstallationPage;
}

bool GuiInstaller::isGuiDevice() const
{
    if (m_deviceType == GuiDevice) {
        return true;
    }
    return false;
}
