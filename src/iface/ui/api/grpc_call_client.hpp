#pragma once

#include <QObject>
#include <grpcpp/grpcpp.h>
#include <memory>

#include "call_api.grpc.pb.h"

class GrpcCallClient : public QObject
{
    Q_OBJECT

public:
    explicit GrpcCallClient(
        std::shared_ptr<grpc::Channel> channel,
        QObject *parent = nullptr
    );

    void subscribe();
    void stop();

    void initiateCall(int device_id);
    void acceptCall();
    void endCall();

signals:
    void callStateChanged(const QString &state);

private:
    void onCallStateChanged(const std::string& state);

    std::unique_ptr<grpc::ClientContext> context_;
    std::unique_ptr<synapsed::api::call::CallSignals::Stub> call_signals_stub_;
    std::unique_ptr<synapsed::api::call::CallActions::Stub> call_actions_stub_;
};
