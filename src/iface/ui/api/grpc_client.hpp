#pragma once

#include <iostream>
#include <memory>
#include <vector>
#include <grpcpp/grpcpp.h>

#include "settings_api.grpc.pb.h"

using synapsed::settings::api::System;

// --- Data Message ---
using ProtoDeviceData = synapsed::settings::api::DeviceData;

class Client {
public:
    Client(std::shared_ptr<grpc::ChannelInterface> channel,
        const std::string& db);

    std::optional<std::vector<ProtoDeviceData>> get_all_devices();
    std::optional<int> get_location_id();
    std::optional<int> get_pref_call_id1();
    std::optional<int> get_pref_call_id2();
    std::optional<int> get_pref_call_id3();
    std::optional<std::vector<ProtoDeviceData>> get_pref_model();
    void set_location_id(int device_id);
    void set_pref_call_id(int num, int device_id);

private:
    std::unique_ptr<System::Stub> stub_;
};