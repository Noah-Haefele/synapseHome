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

int GuiInstaller::currentInstallationPage() const
{
    return m_currentInstallationPage;
}
