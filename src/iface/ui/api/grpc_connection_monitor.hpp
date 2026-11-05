#pragma once

#include <grpcpp/grpcpp.h>
#include <thread>
#include <atomic>
#include <memory>

class GrpcConnectionMonitor
{
public:
    explicit GrpcConnectionMonitor(
        std::shared_ptr<grpc::Channel> channel
    );

    ~GrpcConnectionMonitor();

    void start();
    void stop();

private:
    void run();

    std::shared_ptr<grpc::Channel> channel;
    std::thread worker;
    std::atomic<bool> running{false};
};
