#include "grpc_connection_monitor.hpp"
#include <iostream>

GrpcConnectionMonitor::GrpcConnectionMonitor(
    std::shared_ptr<grpc::Channel> channel
)
    : channel(std::move(channel))
{
}

GrpcConnectionMonitor::~GrpcConnectionMonitor()
{
    stop();
}

void GrpcConnectionMonitor::start()
{
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

        if (newState != state)
        {
            state = newState;

            switch (state)
            {
                case GRPC_CHANNEL_READY:
                    std::cout << "gRPC: CONNECTED\n";
                    break;

                case GRPC_CHANNEL_TRANSIENT_FAILURE:
                    std::cout << "gRPC: DISCONNECTED\n";
                    break;

                case GRPC_CHANNEL_SHUTDOWN:
                    std::cout << "gRPC: SHUTDOWN\n";
                    return;
                
                default:
                    break;
            }
        }
    }
}
