#pragma once

#include <QObject>
#include <QString>
#include <grpcpp/grpcpp.h>
#include <thread>
#include <atomic>
#include <memory>

class GrpcConnectionMonitor : public QObject
{
    Q_OBJECT

public:
    explicit GrpcConnectionMonitor(
        std::shared_ptr<grpc::Channel> channel,
        QObject *parent = nullptr
    );

    ~GrpcConnectionMonitor();

    void start();
    void stop();

signals:
    void softwareStateChanged(const QString &state);

private:
    void run();
    void transmitState(grpc_connectivity_state state);

    std::shared_ptr<grpc::Channel> channel;
    std::thread worker;
    std::atomic<bool> running{false};
};
