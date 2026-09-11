#pragma once

#include <QObject>
#include <QVariantList>
#include <QString>
#include <QtQml/qqmlregistration.h>
#include "grpc_client.hpp"

/**
* @brief Bridges the QML settings view with the backend.
*/
class SettingsBridge : public QObject
{
    Q_OBJECT
    //QML_ELEMENT
    //QML_SINGLETON

    // --- System Settings ---

    Q_PROPERTY(
        QVariantList all_devices
        READ all_devices
        NOTIFY location_id_changed
    )

    Q_PROPERTY(
        QVariantList pref_model
        READ pref_model
        NOTIFY location_id_changed
    )

    Q_PROPERTY(
        int location_id
        READ location_id
        NOTIFY location_id_changed
    )

    Q_PROPERTY(
        int pref1_call_id
        READ pref1_call_id
        NOTIFY pref_call_id_changed
    )

    Q_PROPERTY(
        int pref2_call_id
        READ pref2_call_id
        NOTIFY pref_call_id_changed
    )

    Q_PROPERTY(
        int pref3_call_id
        READ pref3_call_id
        NOTIFY pref_call_id_changed
    )

    Q_PROPERTY(
        QString ip_address
        READ ip_address
        NOTIFY ip_address_changed
    )

    // --- Display Settings ---

    Q_PROPERTY(
        int brightness
        READ brightness
        WRITE set_brightness
        NOTIFY brightness_changed
    )

    Q_PROPERTY(
        int display_time
        READ display_time
        WRITE set_display_time
        NOTIFY display_time_changed
    )

    // --- Audio Settings ---

    Q_PROPERTY(
        QVariantList input_model
        READ input_model
        NOTIFY audio_devices_changed
    )

    Q_PROPERTY(
        QVariantList output_model
        READ output_model
        NOTIFY audio_devices_changed
    )

    Q_PROPERTY(
        int input_device
        READ input_device
        NOTIFY input_device_changed
    )

    Q_PROPERTY(
        int output_device
        READ output_device
        NOTIFY output_device_changed
    )

public:
    explicit SettingsBridge(std::shared_ptr<Client> client, QObject *parent = nullptr);

    // --- System Settings ---

    Q_INVOKABLE void set_location_id(int floorId);
    Q_INVOKABLE void set_pref_call_id(int prefIdx, int floorId);

    QVariantList all_devices() const;
    QVariantList pref_model() const;
    int location_id() const;
    int pref1_call_id() const;
    int pref2_call_id() const;
    int pref3_call_id() const;
    QString ip_address() const;

    // --- Display Settings ---

    int brightness() const;
    int display_time() const;
    void set_brightness(int val);
    void set_display_time(int val);

    // --- Audio Settings ---

    Q_INVOKABLE void onAudioDropdownOpened();

    Q_INVOKABLE void set_sink(const int &id);
    Q_INVOKABLE void set_source(const int &id);

    QVariantList input_model() const;
    QVariantList output_model() const;
    int input_device() const;
    int output_device() const;

private:
    std::shared_ptr<Client> client_;

signals:
    void location_id_changed();
    void pref_call_id_changed();
    void brightness_changed();
    void display_time_changed();
    void audio_devices_changed();
    void pref_call_icon_changed();
    void input_device_changed();
    void output_device_changed();
    void ip_address_changed();
};
