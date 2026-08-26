#pragma once

#include <QObject>
#include <QVariantList>
#include <QString>
#include <QtQml/qqmlregistration.h>
#include "grpc_client.hpp"

/**
* @brief Bridges the QML control grid with the backend.
*/
class ControlGridBridge : public QObject
{
    Q_OBJECT

    // Returns specific icon path for individual prefcall buttons
    Q_PROPERTY(
        QString pref1IconPath
        READ pref1IconPath
        NOTIFY iconChanged
    )

    Q_PROPERTY(
        QString pref2IconPath
        READ pref2IconPath
        NOTIFY iconChanged
    )

    Q_PROPERTY(
        QString pref3IconPath
        READ pref3IconPath
        NOTIFY iconChanged
    )

    // Returns specific label for indibidual prefcall icon
    Q_PROPERTY(
        QString pref1ShortName
        READ pref1ShortName
        NOTIFY iconChanged
    )

    Q_PROPERTY(
        QString pref2ShortName
        READ pref2ShortName
        NOTIFY iconChanged
    )

    Q_PROPERTY(
        QString pref3ShortName
        READ pref3ShortName
        NOTIFY iconChanged
    )

public:
    explicit ControlGridBridge(std::shared_ptr<Client> client, QObject *parent = nullptr);

    QString pref1IconPath() const;
    QString pref2IconPath() const;
    QString pref3IconPath() const;

    QString pref1ShortName() const;
    QString pref2ShortName() const;
    QString pref3ShortName() const;

private:
    std::shared_ptr<Client> client_;

signals:
    void iconChanged();
};
