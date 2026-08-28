#pragma once

#include <QObject>
#include <QVariantList>
#include <QString>
#include <QtQml/qqmlregistration.h>
#include "grpc_client.hpp"

class CallBridge : public QObject
{
    Q_OBJECT

public:
    explicit CallBridge(QObject *parent = nullptr);

signals:
    void callStateChanged(const QString &state);
};
