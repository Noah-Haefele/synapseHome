#include "grpc_connection_monitor.hpp"
#include <iostream>

#include "grpc_connection_monitor.hpp"
#include <iostream>

GrpcConnectionMonitor::GrpcConnectionMonitor(
    std::shared_ptr<grpc::Channel> channel,
    QObject *parent
)
    : QObject(parent),
      channel(std::move(channel))
{
}

GrpcConnectionMonitor::~GrpcConnectionMonitor()
{
    stop();
}

void GrpcConnectionMonitor::start()
{
    auto state = channel->GetState(true);
    GrpcConnectionMonitor::transmitState(state);

    if (running.exchange(true))
        return;

    worker = std::thread(&GrpcConnectionMonitor::run, this);
}

void GrpcConnectionMonitor::stop()
{
    running = false;

    if (worker.joinable())
        worker.join();
}

void GrpcConnectionMonitor::transmitState(grpc_connectivity_state state)
{
    switch (state)
    {
        case GRPC_CHANNEL_READY:
            std::cout << "gRPC: CONNECTED\n";
            emit softwareStateChanged("GRPC:CONNECTED");
            emit reconnected(); // Notify the fronend bridges to fetch data from the server
            break;

        case GRPC_CHANNEL_TRANSIENT_FAILURE:
            std::cout << "gRPC: DISCONNECTED\n";
            emit softwareStateChanged("GRPC:DISCONNECTED");
            break;

        case GRPC_CHANNEL_SHUTDOWN:
            std::cout << "gRPC: SHUTDOWN\n";
            emit softwareStateChanged("GRPC:DISCONNECTED");
            return;
                
        default:
            emit softwareStateChanged("GRPC:DISCONNECTED");
            break;
        
    }
}


void GrpcConnectionMonitor::run()
{
    auto state = channel->GetState(true);

    while (running)
    {
        channel->WaitForStateChange(
            state,
            std::chrono::system_clock::now()
                + std::chrono::seconds(1)
        );

        if (!running)
            break;
        
        auto newState = channel->GetState(true);

        if (newState != state){
            state = newState;
            GrpcConnectionMonitor::transmitState(state);
        }
    }
}
