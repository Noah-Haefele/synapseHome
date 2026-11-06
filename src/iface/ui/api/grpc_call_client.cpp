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

GrpcCallClient::~GrpcCallClient()
{
    stopSubscribe();
}

void GrpcCallClient::startSubscribe()
{
    if (running_)
        return;

    // Eventuell noch vorhandenen Thread aufräumen
    if (subscribeThread_.joinable())
        subscribeThread_.join();

    running_ = true;

    subscribeThread_ = std::thread([this]() {
        subscribe();
    });
}

void GrpcCallClient::stopSubscribe()
{
    running_ = false;

    {
        std::lock_guard<std::mutex> lock(contextMutex_);

        if (activeContext_) {
            activeContext_->TryCancel();
        }
    }

    if (subscribeThread_.joinable()) {
        subscribeThread_.join();
    }
}

void GrpcCallClient::subscribe()
{
    SubscribeRequest request;
    request.set_topic("on_call_state_changed");

    grpc::ClientContext context;

    {
        std::lock_guard<std::mutex> lock(contextMutex_);
        activeContext_ = &context;
    }

    std::unique_ptr<grpc::ClientReader<Event>> reader(
        call_signals_stub_->Subscribe(&context, request)
    );

    Event event;

    while (running_ && reader->Read(&event)) {
        if (event.event_type() == "on_call_state_changed") {
            onCallStateChanged(event.state());
        }
    }

    grpc::Status status = reader->Finish();

    {
        std::lock_guard<std::mutex> lock(contextMutex_);
        activeContext_ = nullptr;
    }

    if (!status.ok()) {
        std::cerr << "Subscribe ended: "
                  << status.error_message()
                  << std::endl;
    }

    running_ = false;
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

void GrpcCallClient::initiateCallAll() {
    google::protobuf::Empty request;
    google::protobuf::Empty reply;
    grpc::ClientContext context;

    grpc::Status status = call_actions_stub_->All(&context, request, &reply);
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

void GrpcCallClient::openDoor()
{
    google::protobuf::Empty request;
    google::protobuf::Empty reply;
    grpc::ClientContext context;

    grpc::Status status = call_actions_stub_->OpenDoor(&context, request, &reply);

    if (!status.ok()) {
        std::cerr << "OpenDoor gRPC error: " << status.error_message() << std::endl;
    }
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
