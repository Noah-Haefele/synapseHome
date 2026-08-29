#include "grpc_call_client.hpp"

#include <iostream>

using synapsed::api::call::Event;
using synapsed::api::call::CallSignals;
using synapsed::api::call::SubscribeRequest;

GrpcCallClient::GrpcCallClient(
    std::shared_ptr<grpc::Channel> channel,
    QObject *parent
)
    : QObject(parent),
      call_signals_stub_(CallSignals::NewStub(channel))
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
    std::cout << "Call icon changed!" << std::endl;

    emit callStateChanged(QString::fromStdString(state));
}
