#include <iostream>
#include <string>
#include <QVariantMap>

#include "settings_bridge.hpp"


SettingsBridge::SettingsBridge(std::shared_ptr<Client> client,
    QObject *parent)
    : QObject(parent), client_(std::move(client))
{
}

void print(const std::string &msg)
{
    std::cout << msg << "\n";
}

// --- System Settings ---

Q_INVOKABLE void SettingsBridge::set_location_id(int device_id)
{
    client_->set_location_id(device_id);
    // Emit both
    // Location id because of a new preference model available
    // Pref call id because of some invalid and then deleted preferences
    emit location_id_changed();
    emit pref_call_id_changed();
}

Q_INVOKABLE void SettingsBridge::set_pref_call_id(int num, int device_id)
{
    client_->set_pref_call_id(num, device_id);
    // Emit pref call id because of some pref which might be already on this id and hasEmit pref call id because of some pref which might be already on this id and hasEmit pref call id because of some pref which might be already on this id and hasEmit pref call id because of some pref which might be already on this id and hasEmit pref call id because of some pref which might be already on this id and has
    emit pref_call_id_changed();
}

QVariantList SettingsBridge::all_devices() const
{
    QVariantList list;

    if (!client_) return list;

    auto devices = client_->get_all_devices();
    if (!devices.has_value()) {
        return list;
    }

    for (const auto& device : devices.value()) {
        QVariantMap map;
        map["device_name"] = QString::fromStdString(device.device_name());
        map["device_short_name"] = QString::fromStdString(device.device_short_name());
        map["device_id"] = device.device_id();
        list.append(map);
    }

    return list;
}

QVariantList SettingsBridge::pref_model() const
{
    QVariantList list;

    if (!client_) return list;

    auto devices = client_->get_pref_model();
    if (!devices.has_value()) {
        return list;
    }

    for (const auto& device : devices.value()) {
        QVariantMap map;
        map["device_name"] = QString::fromStdString(device.device_name());
        map["device_short_name"] = QString::fromStdString(device.device_short_name());
        map["device_id"] = device.device_id();
        list.append(map);
    }

    return list;
}

int SettingsBridge::location_id() const
{
    auto location_id = client_->get_location_id();
    if (location_id == std::nullopt) {
        return -1;
    }

    return *location_id;
}

int SettingsBridge::pref1_call_id() const
{
    auto device_id = client_->get_pref1_call_id();
    if (device_id == std::nullopt) {
        return -1;
    }

    return *device_id;
}

int SettingsBridge::pref2_call_id() const
{
    auto device_id = client_->get_pref2_call_id();
    if (device_id == std::nullopt) {
        return -1;
    }

    return *device_id;
}

int SettingsBridge::pref3_call_id() const
{
    auto device_id = client_->get_pref3_call_id();
    if (device_id == std::nullopt) {
        return -1;
    }

    return *device_id;
}

// --- Display Settings ---

int SettingsBridge::brightness() const
{
    auto val = client_->get_brightness();
    if (val == std::nullopt) {
        return -1;
    }

    return *val;
}

int SettingsBridge::display_time() const
{
    auto val = client_->get_display_time();
    if (val == std::nullopt) {
        return -1;
    }

    return *val;
}

void SettingsBridge::set_brightness(int val)
{
    client_->set_brightness(val);
}

void SettingsBridge::set_display_time(int val)
{
    client_->set_display_time(val);
}

// --- Audio Settings ---

Q_INVOKABLE void SettingsBridge::onAudioDropdownOpened()
{
    emit settings_changed();
}

Q_INVOKABLE void SettingsBridge::set_sink(const QString &id)
{
    //print(std::format("setSink: {}", id));
    print("setSink");
}

Q_INVOKABLE void SettingsBridge::set_source(const QString &id)
{
    //print(std::format("setSource: {}", id));
    print("setSource");
}

QVariantList SettingsBridge::input_model() const
{
    print("inptuModel");
    return {};
}

QVariantList SettingsBridge::output_model() const
{
    print("outputModel");
    return {};
}

QString SettingsBridge::input_device() const
{
    print("inputDevice");
    return "";
}

QString SettingsBridge::output_device() const
{
    print("outputDevice");
    return "";
}
