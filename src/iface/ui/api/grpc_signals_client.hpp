#pragma once

#include <QObject>
#include <grpcpp/grpcpp.h>
#include <memory>

#include "events_api.grpc.pb.h"

class SignalClient : public QObject
{
    Q_OBJECT

public:
    explicit SignalClient(
        std::shared_ptr<grpc::Channel> channel,
        QObject *parent = nullptr
    );

    void subscribe();
    void stop();

signals:
    void callIconChanged();

private:
    void on_call_icon_changed();

    std::unique_ptr<grpc::ClientContext> context_;
    std::unique_ptr<synapsed::api::events::EventService::Stub> stub_;
};
