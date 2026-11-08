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

public:
    explicit GuiInstaller(QObject *parent = nullptr);

    Q_INVOKABLE void nextInstallationPage();

    int currentInstallationPage() const;

private:
    int m_currentInstallationPage = 1;

signals:
    void currentInstallationPageChanged();
};
