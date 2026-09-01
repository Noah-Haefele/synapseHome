#include <optional>
#include <grpcpp/grpcpp.h>
#include <google/protobuf/empty.pb.h>

#include "settings_api.grpc.pb.h"
#include "pref_api.grpc.pb.h"
#include "grpc_client.hpp"

// --- Settings Api ---
using synapsed::api::settings::System;
using synapsed::api::settings::Display;

// --- Preference Api ---
using synapsed::api::pref::PrefIconPaths;
using synapsed::api::pref::PrefCallIds;
using synapsed::api::pref::PrefShortNames;
using synapsed::api::pref::PrefModels;

// --- Data Message ---
using ProtoDeviceData = synapsed::api::helper::DeviceData;

// --- Request Messages ---
// System Settings
using synapsed::api::settings::SetLocationIdRequest;
// Display Settings
using synapsed::api::settings::SetBrightnessRequest;
using synapsed::api::settings::SetDisplayTimeRequest;
// Pref Ids
using synapsed::api::pref::SetPrefCallIdRequest;
using synapsed::api::pref::GetPrefCallIdRequest;

// --- Reply Messages ---
// System Settings
using synapsed::api::settings::GetAllDevicesReply;
using synapsed::api::settings::GetLocationIdReply;
// Display Settings
using synapsed::api::settings::GetBrightnessReply;
using synapsed::api::settings::GetDisplayTimeReply;
// Pref Paths
using synapsed::api::pref::GetPref1IconPathReply;
using synapsed::api::pref::GetPref2IconPathReply;
using synapsed::api::pref::GetPref3IconPathReply;
// Pref Ids
using synapsed::api::pref::GetPrefCallIdReply;
// Pref short Labels
using synapsed::api::pref::GetPref1ShortNameReply;
using synapsed::api::pref::GetPref2ShortNameReply;
using synapsed::api::pref::GetPref3ShortNameReply;
// Pref Models
using synapsed::api::pref::GetPrefModelReply;

Client::Client(std::shared_ptr<grpc::ChannelInterface> channel,
               const std::string& db)
    : system_stub_(System::NewStub(channel)),
      display_stub_(Display::NewStub(channel)),
      pref_icon_paths_stub_(PrefIconPaths::NewStub(channel)),
      pref_call_ids_stub_(PrefCallIds::NewStub(channel)),
      pref_short_names_stub_(PrefShortNames::NewStub(channel)),
      pref_models_stub_(PrefModels::NewStub(channel)) {
}

// --- System Settings ---

std::optional<std::vector<ProtoDeviceData>> Client::get_all_devices() {
    google::protobuf::Empty request;
    GetAllDevicesReply reply;
    grpc::ClientContext context;

    grpc::Status status = system_stub_->GetAllDevices(&context, request, &reply);
    if (status.ok()) {
        // Convert repeatedfield of grpc into vec
        return std::vector<ProtoDeviceData>(reply.devices().begin(), reply.devices().end());
    }
    return std::nullopt;
}

std::optional<int> Client::get_location_id() {
    google::protobuf::Empty request;
    GetLocationIdReply reply;
    grpc::ClientContext context;

    grpc::Status status = system_stub_->GetLocationId(&context, request, &reply);
    if (status.ok()) {
        return reply.device_id();
    }
    return std::nullopt;
}

std::optional<int> Client::get_pref_call_id(int num) {
    GetPrefCallIdRequest request;
    request.set_num(num);
    GetPrefCallIdReply reply;
    grpc::ClientContext context;

    grpc::Status status = pref_call_ids_stub_->GetPrefCallId(&context, request, &reply);
    if (status.ok()) {
        return reply.device_id();
    }
    return std::nullopt;
}

std::optional<std::vector<ProtoDeviceData>> Client::get_pref_model() {
    google::protobuf::Empty request;
    GetPrefModelReply reply;
    grpc::ClientContext context;

    grpc::Status status = pref_models_stub_->GetPrefModel(&context, request, &reply);
    if (status.ok()) {
        // Convert repeatedfield of grpc into vec
        return std::vector<ProtoDeviceData>(reply.devices().begin(), reply.devices().end());
    }
    return std::nullopt;
}

void Client::set_location_id(int device_id) {
    SetLocationIdRequest request;
    request.set_device_id(device_id);

    google::protobuf::Empty reply;
    grpc::ClientContext context;

    grpc::Status status = system_stub_->SetLocationId(&context, request, &reply);
}

void Client::set_pref_call_id(int num, int device_id) {
    SetPrefCallIdRequest request;
    request.set_num(num);
    request.set_device_id(device_id);

    google::protobuf::Empty reply;
    grpc::ClientContext context;

    grpc::Status status = pref_call_ids_stub_->SetPrefCallId(&context, request, &reply);
}

// --- Display Settings ---

std::optional<int> Client::get_brightness() {
    google::protobuf::Empty request;
    GetBrightnessReply reply;
    grpc::ClientContext context;

    grpc::Status status = display_stub_->GetBrightness(&context, request, &reply);
    if (status.ok()) {
        return reply.val();
    }
    return std::nullopt;
}

std::optional<int> Client::get_display_time() {
    google::protobuf::Empty request;
    GetDisplayTimeReply reply;
    grpc::ClientContext context;

    grpc::Status status = display_stub_->GetDisplayTime(&context, request, &reply);
    if (status.ok()) {
        return reply.val();
    }
    return std::nullopt;
}

void Client::set_brightness(int val) {
    SetBrightnessRequest request;
    request.set_val(val);

    google::protobuf::Empty reply;
    grpc::ClientContext context;

    grpc::Status status = display_stub_->SetBrightness(&context, request, &reply);
}

void Client::set_display_time(int val) {
    SetDisplayTimeRequest request;
    request.set_val(val);

    google::protobuf::Empty reply;
    grpc::ClientContext context;

    grpc::Status status = display_stub_->SetDisplayTime(&context, request, &reply);
}

// --- Control Grid ---

std::optional<std::string> Client::get_pref1_icon_path() {
    google::protobuf::Empty request;
    GetPref1IconPathReply reply;
    grpc::ClientContext context;

    grpc::Status status = pref_icon_paths_stub_->GetPref1IconPath(&context, request, &reply);
    if (status.ok()) {
        return reply.path();
    }
    return std::nullopt;
}

std::optional<std::string> Client::get_pref2_icon_path() {
    google::protobuf::Empty request;
    GetPref2IconPathReply reply;
    grpc::ClientContext context;

    grpc::Status status = pref_icon_paths_stub_->GetPref2IconPath(&context, request, &reply);
    if (status.ok()) {
        return reply.path();
    }
    return std::nullopt;
}

std::optional<std::string> Client::get_pref3_icon_path() {
    google::protobuf::Empty request;
    GetPref3IconPathReply reply;
    grpc::ClientContext context;

    grpc::Status status = pref_icon_paths_stub_->GetPref3IconPath(&context, request, &reply);
    if (status.ok()) {
        return reply.path();
    }
    return std::nullopt;
}

std::optional<std::string> Client::get_pref1_short_name() {
    google::protobuf::Empty request;
    GetPref1ShortNameReply reply;
    grpc::ClientContext context;

    grpc::Status status = pref_short_names_stub_->GetPref1ShortName(&context, request, &reply);
    if (status.ok()) {
        return reply.name();
    }
    return std::nullopt;
}

std::optional<std::string> Client::get_pref2_short_name() {
    google::protobuf::Empty request;
    GetPref2ShortNameReply reply;
    grpc::ClientContext context;

    grpc::Status status = pref_short_names_stub_->GetPref2ShortName(&context, request, &reply);
    if (status.ok()) {
        return reply.name();
    }
    return std::nullopt;
}

std::optional<std::string> Client::get_pref3_short_name() {
    google::protobuf::Empty request;
    GetPref3ShortNameReply reply;
    grpc::ClientContext context;

    grpc::Status status = pref_short_names_stub_->GetPref3ShortName(&context, request, &reply);
    if (status.ok()) {
        return reply.name();
    }
    return std::nullopt;
}
