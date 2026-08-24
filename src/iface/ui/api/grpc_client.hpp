#pragma once

#include <iostream>
#include <memory>
#include <vector>
#include <grpcpp/grpcpp.h>

#include "settings_api.grpc.pb.h"

using synapsed::settings::api::System;
using synapsed::settings::api::Display;

// --- Data Message ---
using ProtoDeviceData = synapsed::settings::api::DeviceData;

/**
 * @brief Client for communicating with the backend via gRPC.
 *
 * Provides methods for retrieving and modifying system settings
 * through the backend gRPC service.
 */
class Client {
public:
    Client(std::shared_ptr<grpc::ChannelInterface> channel,
        const std::string& db);

    // --- System Settings ---
    
    std::optional<std::vector<ProtoDeviceData>> get_all_devices();
    std::optional<int> get_location_id();
    std::optional<int> get_pref_call_id1();
    std::optional<int> get_pref_call_id2();
    std::optional<int> get_pref_call_id3();
    std::optional<std::vector<ProtoDeviceData>> get_pref_model();
    void set_location_id(int device_id);
    void set_pref_call_id(int num, int device_id);

    // --- Display Settings ---
    std::optional<int> get_brightness();
    std::optional<int> get_display_time();
    void set_brightness(int val);
    void set_display_time(int val);

private:
    std::unique_ptr<System::Stub> system_stub_;
    std::unique_ptr<Display::Stub> display_stub_;
};