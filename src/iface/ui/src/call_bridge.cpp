#include "call_bridge.hpp"
#include <iostream>

CallBridge::CallBridge(
    GrpcCallClient *grpc_call_client,
    QObject *parent
)
    : QObject(parent),
      grpc_call_client_(grpc_call_client)
{
}

void CallBridge::initiateCall(int device_id)
{
    grpc_call_client_->initiateCall(device_id);
}

void CallBridge::acceptCall()
{
    grpc_call_client_->acceptCall();
}

void CallBridge::endCall()
{
    grpc_call_client_->endCall();
}

QString CallBridge::destinationLabel() const
{
    auto call_label = grpc_call_client_->getCallLabel();

    if (call_label == std::nullopt) {
        std::cerr << "GetCallLabel gRPC error" << std::endl;
        return QStringLiteral("Unable to load call information");
    }
    return QString::fromStdString(*call_label);
}
