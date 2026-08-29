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
