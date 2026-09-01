#pragma once

#include <QObject>
#include <QVariantList>
#include <QString>
#include <QtQml/qqmlregistration.h>
#include "grpc_call_client.hpp"

class CallBridge : public QObject
{
    Q_OBJECT

    Q_PROPERTY(
        QString destinationLabel
        READ destinationLabel
        NOTIFY callStateChanged
    )

public:
    explicit CallBridge(GrpcCallClient *grpc_call_client, QObject *parent = nullptr);

    Q_INVOKABLE void initiateCall(int device_id);
    Q_INVOKABLE void acceptCall();
    Q_INVOKABLE void endCall();
    QString destinationLabel() const;

private:
    GrpcCallClient *grpc_call_client_;

signals:
    void callStateChanged(const QString &state);
};
