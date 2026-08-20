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
        NOTIFY settings_changed
    )

    Q_PROPERTY(
        QVariantList pref_model
        READ pref_model
        NOTIFY settings_changed
    )

    Q_PROPERTY(
        int location_id
        READ location_id
        NOTIFY settings_changed
    )

    Q_PROPERTY(
        int pref_call_id1
        READ pref_call_id1
        NOTIFY settings_changed
    )

    Q_PROPERTY(
        int pref_call_id2
        READ pref_call_id2
        NOTIFY settings_changed
    )

    Q_PROPERTY(
        int pref_call_id3
        READ pref_call_id3
        NOTIFY settings_changed
    )

    // --- Display Settings ---

    Q_PROPERTY(
        int brightness
        READ brightness
        WRITE set_brightness
        NOTIFY settings_changed
    )

    Q_PROPERTY(
        int display_time
        READ display_time
        WRITE set_display_time
        NOTIFY settings_changed
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
        QString input_device
        READ input_device
        NOTIFY settings_changed
    )

    Q_PROPERTY(
        QString output_device
        READ output_device
        NOTIFY settings_changed
    )

public:
    explicit SettingsBridge(std::shared_ptr<Client> client, QObject *parent = nullptr);

    // --- System Settings ---

    Q_INVOKABLE void set_location_id(int floorId);
    Q_INVOKABLE void set_pref_call_id(int prefIdx, int floorId);

    QVariantList all_devices() const;
    QVariantList pref_model() const;
    int location_id() const;
    int pref_call_id1() const;
    int pref_call_id2() const;
    int pref_call_id3() const;

    // --- Display Settings ---

    int brightness() const;
    int display_time() const;
    void set_brightness(int val);
    void set_display_time(int val);

    // --- Audio Settings ---

    Q_INVOKABLE void onAudioDropdownOpened();

    Q_INVOKABLE void set_sink(const QString &id);
    Q_INVOKABLE void set_source(const QString &id);

    QVariantList input_model() const;
    QVariantList output_model() const;
    QString input_device() const;
    QString output_device() const;

private:
    std::shared_ptr<Client> client_;

signals:
    void settings_changed();
    void audio_devices_changed();
};
