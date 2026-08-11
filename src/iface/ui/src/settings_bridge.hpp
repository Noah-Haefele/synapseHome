#pragma once

#include <QObject>
#include <QVariantList>
#include <QtQml/qqmlregistration.h>

/**
* @brief Bridges the QML settings view with the backend.
*/
class SettingsBridge : public QObject
{
    Q_OBJECT
    QML_ELEMENT

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

public:
    Q_INVOKABLE void setLocationIdx(int floorId);
    Q_INVOKABLE void setPrefCallIdx(int prefIdx, int floorId);

    QVariantList allFloors() const;
    QVariantList prefModel() const;
    int locationIdx() const;
    int prefCallIdx1() const;
    int prefCallIdx2() const;
    int prefCallIdx3() const;

signals:
    void settingsChanged();
};