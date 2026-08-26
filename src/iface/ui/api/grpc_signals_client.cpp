#include "grpc_signals_client.hpp"

#include <iostream>

using synapsed::api::events::Event;
using synapsed::api::events::EventService;
using synapsed::api::events::SubscribeRequest;

SignalClient::SignalClient(
    std::shared_ptr<grpc::Channel> channel,
    QObject *parent
)
    : QObject(parent),
      stub_(EventService::NewStub(channel))
{
}

void SignalClient::subscribe()
{
    SubscribeRequest request;
    request.set_topic("on_call_icon_changed");

    context_ = std::make_unique<grpc::ClientContext>();

    std::unique_ptr<grpc::ClientReader<Event>> reader(
        stub_->Subscribe(context_.get(), request)
    );

    Event event;

    while (reader->Read(&event)) {
        if (event.event_type() == "on_call_icon_changed") {
            on_call_icon_changed();
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

void SignalClient::stop()
{
    if (context_) {
        context_->TryCancel();
    }
}

void SignalClient::on_call_icon_changed()
{
    std::cout << "Call icon changed!" << std::endl;

    emit callIconChanged();
}
