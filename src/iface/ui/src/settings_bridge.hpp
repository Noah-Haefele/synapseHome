#pragma once

#include <QObject>
#include <QVariantList>
#include <QString>
#include <QtQml/qqmlregistration.h>

/**
* @brief Bridges the QML settings view with the backend.
*/
class SettingsBridge : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON

    // --- System Settings ---

    Q_PROPERTY(
        QVariantList allFloors
        READ allFloors
        NOTIFY settingsChanged
    )

    Q_PROPERTY(
        QVariantList prefModel
        READ prefModel
        NOTIFY settingsChanged
    )

    Q_PROPERTY(
        int locationIdx
        READ locationIdx
        NOTIFY settingsChanged
    )

    Q_PROPERTY(
        int prefCallIdx1
        READ prefCallIdx1
        NOTIFY settingsChanged
    )

    Q_PROPERTY(
        int prefCallIdx2
        READ prefCallIdx2
        NOTIFY settingsChanged
    )

    Q_PROPERTY(
        int prefCallIdx3
        READ prefCallIdx3
        NOTIFY settingsChanged
    )

    // --- Display Settings ---

    Q_PROPERTY(
        int brightness
        READ brightness
        WRITE setBrightness
        NOTIFY settingsChanged
    )

    Q_PROPERTY(
        int displayTime
        READ displayTime
        WRITE setDisplayTime
        NOTIFY settingsChanged
    )

    // --- Audio Settings ---

    Q_PROPERTY(
        QVariantList inputModel
        READ inputModel
        NOTIFY audioDevicesChanged
    )

    Q_PROPERTY(
        QVariantList outputModel
        READ outputModel
        NOTIFY audioDevicesChanged
    )

    Q_PROPERTY(
        QString inputDevice
        READ inputDevice
        NOTIFY settingsChanged
    )

    Q_PROPERTY(
        QString outputDevice
        READ outputDevice
        NOTIFY settingsChanged
    )

public:
    explicit SettingsBridge(QObject *parent = nullptr);

    // --- System Settings ---

    Q_INVOKABLE void setLocationIdx(int floorId);
    Q_INVOKABLE void setPrefCallIdx(int prefIdx, int floorId);

    QVariantList allFloors() const;
    QVariantList prefModel() const;
    int locationIdx() const;
    int prefCallIdx1() const;
    int prefCallIdx2() const;
    int prefCallIdx3() const;

    // --- Display Settings ---

    int brightness() const;
    int displayTime() const;
    void setBrightness(int val);
    void setDisplayTime(int val);

    // --- Audio Settings ---

    Q_INVOKABLE void onAudioDropdownOpened();

    Q_INVOKABLE void setSink(const QString &id);
    Q_INVOKABLE void setSource(const QString &id);

    QVariantList inputModel() const;
    QVariantList outputModel() const;
    QString inputDevice() const;
    QString outputDevice() const;

signals:
    void settingsChanged();
    void audioDevicesChanged();
};
