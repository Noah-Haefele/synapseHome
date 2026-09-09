#pragma once

#include <grpcpp/grpcpp.h>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "settings_api.grpc.pb.h"
#include "pref_api.grpc.pb.h"

// --- Settings Api ---
using synapsed::api::settings::System;
using synapsed::api::settings::Display;
using synapsed::api::settings::Audio;

// --- Preference Api ---
using synapsed::api::pref::PrefIconPaths;
using synapsed::api::pref::PrefCallIds;
using synapsed::api::pref::PrefShortNames;
using synapsed::api::pref::PrefModels;

// --- Data Message ---
using ProtoDeviceData = synapsed::api::helper::DeviceData;
using ProtoSinkSourceData = synapsed::api::helper::SinkSourceData;

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
    std::optional<int> get_pref_call_id(int num);
    std::optional<std::vector<ProtoDeviceData>> get_pref_model();
    void set_location_id(int device_id);
    void set_pref_call_id(int num, int device_id);

    // --- Display Settings ---
    std::optional<int> get_brightness();
    std::optional<int> get_display_time();
    void set_brightness(int val);
    void set_display_time(int val);

    // --- Audio Settings ---
    std::optional<std::vector<ProtoSinkSourceData>> get_sink_model() const;
    std::optional<std::vector<ProtoSinkSourceData>> get_source_model() const;
    std::optional<int> get_default_sink_id() const;
    std::optional<int> get_default_source_id() const;
    void set_sink_source(int id);

    // --- Control Grid ---
    std::optional<std::string> get_pref1_icon_path();
    std::optional<std::string> get_pref2_icon_path();
    std::optional<std::string> get_pref3_icon_path();
    std::optional<std::string> get_pref1_short_name();
    std::optional<std::string> get_pref2_short_name();
    std::optional<std::string> get_pref3_short_name();

private:
    std::unique_ptr<System::Stub> system_stub_;
    std::unique_ptr<Display::Stub> display_stub_;
    std::unique_ptr<Audio::Stub> audio_stub_;
    std::unique_ptr<PrefIconPaths::Stub> pref_icon_paths_stub_;
    std::unique_ptr<PrefCallIds::Stub> pref_call_ids_stub_;
    std::unique_ptr<PrefShortNames::Stub> pref_short_names_stub_;
    std::unique_ptr<PrefModels::Stub> pref_models_stub_;
};
