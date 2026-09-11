#include <optional>
#include <QVariantMap>

#include "control_grid_bridge.hpp"

ControlGridBridge::ControlGridBridge(std::shared_ptr<Client> client,
    QObject *parent)
    : QObject(parent), client_(std::move(client))
{
}

QString ControlGridBridge::pref1IconPath() const
{
    auto path = client_->get_pref1_icon_path();
    if (path == std::nullopt) {
        return QString::fromStdString("");
    }

    return QString::fromStdString(*path);
}

QString ControlGridBridge::pref2IconPath() const
{
    auto path = client_->get_pref2_icon_path();
    if (path == std::nullopt) {
        return QString::fromStdString("");
    }

    return QString::fromStdString(*path);
}

QString ControlGridBridge::pref3IconPath() const
{
    auto path = client_->get_pref3_icon_path();
    if (path == std::nullopt) {
        return QString::fromStdString("");
    }

    return QString::fromStdString(*path);
}

QString ControlGridBridge::pref1ShortName() const
{
    auto name = client_->get_pref1_short_name();
    if (name == std::nullopt) {
        return QString::fromStdString("?");
    }

    return QString::fromStdString(*name);
}

QString ControlGridBridge::pref2ShortName() const
{
    auto name = client_->get_pref2_short_name();
    if (name == std::nullopt) {
        return QString::fromStdString("?");
    }

    return QString::fromStdString(*name);
}

QString ControlGridBridge::pref3ShortName() const
{
    auto name = client_->get_pref3_short_name();
    if (name == std::nullopt) {
        return QString::fromStdString("?");
    }

    return QString::fromStdString(*name);
}

int ControlGridBridge::getPrefCallId(int num) const
{
    auto device_id = client_->get_pref_call_id(num);
    if (device_id == std::nullopt) {
        return -1;
    }

    return *device_id;
}
