#include "grpc_call_client.hpp"

#include <google/protobuf/empty.pb.h>
#include <iostream>

using synapsed::api::call::CallSignals;
using synapsed::api::call::CallActions;
using synapsed::api::call::CallHelpers;

// --- Call Signals ---
using synapsed::api::call::Event;
using synapsed::api::call::SubscribeRequest;

// --- Call Actions ---
using synapsed::api::call::InitiateRequest;

// --- Call Helpers ---
using synapsed::api::call::GetCallLabelReply;

GrpcCallClient::GrpcCallClient(
    std::shared_ptr<grpc::Channel> channel,
    QObject *parent
)
    : QObject(parent),
      call_signals_stub_(CallSignals::NewStub(channel)),
      call_actions_stub_(CallActions::NewStub(channel)),
      call_helpers_stub_(CallHelpers::NewStub(channel))
{
}

void GrpcCallClient::subscribe()
{
    SubscribeRequest request;
    request.set_topic("on_call_state_changed");

    context_ = std::make_unique<grpc::ClientContext>();

    std::unique_ptr<grpc::ClientReader<Event>> reader(
        call_signals_stub_->Subscribe(context_.get(), request)
    );

    Event event;

    while (reader->Read(&event)) {
        if (event.event_type() == "on_call_state_changed") {
            onCallStateChanged(event.state());
        }
    }

    grpc::Status status = reader->Finish();

    if (!status.ok()) {
        std::cerr << "Subscribe ended: "
                  << status.error_message()
                  << std::endl;
    }

    context_.reset();
}

void GrpcCallClient::stop()
{
    if (context_) {
        context_->TryCancel();
    }
}

void GrpcCallClient::onCallStateChanged(const std::string& state)
{
    emit callStateChanged(QString::fromStdString(state));
}

void GrpcCallClient::initiateCall(int device_id) {
    InitiateRequest request;
    request.set_device_id(device_id);

    google::protobuf::Empty reply;
    grpc::ClientContext context;

    grpc::Status status = call_actions_stub_->Initiate(&context, request, &reply);
}

void GrpcCallClient::acceptCall() {
    google::protobuf::Empty request;
    google::protobuf::Empty reply;
    grpc::ClientContext context;

    grpc::Status status = call_actions_stub_->Accept(&context, request, &reply);
}

void GrpcCallClient::endCall() {
    google::protobuf::Empty request;
    google::protobuf::Empty reply;
    grpc::ClientContext context;

    grpc::Status status = call_actions_stub_->End(&context, request, &reply);
}

std::optional<std::string> GrpcCallClient::getCallLabel() const
{
    google::protobuf::Empty request;
    GetCallLabelReply reply;
    grpc::ClientContext context;

    grpc::Status status = call_helpers_stub_->GetCallLabel(&context, request, &reply);
    if (!status.ok()) {
        return std::nullopt;
        std::cerr << "GetCallLabel gRPC error: "
            << status.error_message()
            << std::endl;
    }
    return reply.call_label();
}
