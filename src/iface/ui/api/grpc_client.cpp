#include <iostream>
#include <memory>
#include <vector>
#include <optional>
#include <grpcpp/grpcpp.h>
#include <google/protobuf/empty.pb.h>

#include "settings_api.grpc.pb.h"
#include "grpc_client.hpp"

using synapsed::settings::api::System;
using synapsed::settings::api::Display;

// --- Data Message ---
using ProtoDeviceData = synapsed::settings::api::DeviceData;

// --- Request Messages ---
// System
using synapsed::settings::api::SetLocationIdRequest;
using synapsed::settings::api::SetPrefCallIdRequest;
// Display
using synapsed::settings::api::SetBrightnessRequest;
using synapsed::settings::api::SetDisplayTimeRequest;

// --- Reply Messages ---
// System
using synapsed::settings::api::GetAllDevicesReply;
using synapsed::settings::api::GetPrefModelReply;
using synapsed::settings::api::GetLocationIdReply;
using synapsed::settings::api::GetPrefCallId1Reply;
using synapsed::settings::api::GetPrefCallId2Reply;
using synapsed::settings::api::GetPrefCallId3Reply;
// Display
using synapsed::settings::api::GetBrightnessReply;
using synapsed::settings::api::GetDisplayTimeReply;

Client::Client(std::shared_ptr<grpc::ChannelInterface> channel,
               const std::string& db)
    : system_stub_(System::NewStub(channel)),
      display_stub_(Display::NewStub(channel)) {
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

std::optional<int> Client::get_pref_call_id1() {
    google::protobuf::Empty request;
    GetPrefCallId1Reply reply;
    grpc::ClientContext context;

    grpc::Status status = system_stub_->GetPrefCallId1(&context, request, &reply);
    if (status.ok()) {
        return reply.device_id();
    }
    return std::nullopt;
}

std::optional<int> Client::get_pref_call_id2() {
    google::protobuf::Empty request;
    GetPrefCallId2Reply reply;
    grpc::ClientContext context;

    grpc::Status status = system_stub_->GetPrefCallId2(&context, request, &reply);
    if (status.ok()) {
        return reply.device_id();
    }
    return std::nullopt;
}

std::optional<int> Client::get_pref_call_id3() {
    google::protobuf::Empty request;
    GetPrefCallId3Reply reply;
    grpc::ClientContext context;

    grpc::Status status = system_stub_->GetPrefCallId3(&context, request, &reply);
    if (status.ok()) {
        return reply.device_id();
    }
    return std::nullopt;
}

std::optional<std::vector<ProtoDeviceData>> Client::get_pref_model() {
    google::protobuf::Empty request;
    GetPrefModelReply reply;
    grpc::ClientContext context;

    grpc::Status status = system_stub_->GetPrefModel(&context, request, &reply);
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

    grpc::Status status = system_stub_->SetPrefCallId(&context, request, &reply);
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

