#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJ_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_DIR="$PROJ_DIR/config"
DEVICES_JSON="$CONFIG_DIR/devices.json"
DOOR_DEVICE_JSON="$PROJ_DIR/internal/door_device.json"
INTERNAL_SETTINGS_JSON="$PROJ_DIR/internal/settings.json"


echo "================================================="
echo "   Doorstation Interactive Configuration Setup   "
echo "================================================="

if [ ! -f "$DEVICES_JSON" ]; then
    echo "Error: $DEVICES_JSON not found!"
    exit 1
fi

echo ""
echo "Configured Door Devices in devices.json:"
echo "----------------------------------------"
if command -v jq &> /dev/null; then
    jq '.door_devices' "$DEVICES_JSON"
else
    cat "$DEVICES_JSON"
fi
echo ""

# Ask for location_id
while true; do
    read -rp "Enter Door Device Location ID (deviceId from devices.json): " LOCATION_ID
    if [[ "$LOCATION_ID" =~ ^[0-9]+$ ]]; then
        if grep -q "\"deviceId\":[[:space:]]*$LOCATION_ID" "$DEVICES_JSON"; then
            echo "✓ Location ID $LOCATION_ID matched in devices.json!"
            break
        else
            echo "⚠️ Warning: Device ID $LOCATION_ID was not found in devices.json."
            read -rp "Use ID $LOCATION_ID anyway? [y/N]: " CONFIRM
            if [[ "$CONFIRM" =~ ^[Yy]$ ]]; then
                break
            fi
        fi
    else
        echo "Please enter a valid numeric ID."
    fi
done

read -rp "Enter Buzzer Pin (Door Opener Relay) [default: 18]: " BUZZER_PIN
BUZZER_PIN=${BUZZER_PIN:-18}

read -rp "Enter Ring Pin (Doorbell Button) [default: 23]: " RING_PIN
RING_PIN=${RING_PIN:-23}

read -rp "Enter Door Open Pin (Inside Unlock Button) [default: 24]: " OPEN_PIN
OPEN_PIN=${OPEN_PIN:-24}

read -rp "Enter Buzz Duration in milliseconds [default: 3000]: " BUZZ_DURATION
BUZZ_DURATION=${BUZZ_DURATION:-3000}

# Save config/door_device.json
mkdir -p "$CONFIG_DIR"
cat << EOF > "$DOOR_DEVICE_JSON"
{
  "location_id": $LOCATION_ID,
  "buzzer_pin": $BUZZER_PIN,
  "ring_pin": $RING_PIN,
  "open_pin": $OPEN_PIN,
  "buzz_duration_ms": $BUZZ_DURATION
}
EOF

# Update internal/settings.json location_id
INTERNAL_DIR="$(dirname "$INTERNAL_SETTINGS_JSON")"
mkdir -p "$INTERNAL_DIR"
cat << EOF > "$INTERNAL_SETTINGS_JSON"
{
  "location_id": $LOCATION_ID,
  "pref1_call_id": -1,
  "pref2_call_id": -1,
  "pref3_call_id": -1
}
EOF

echo ""
echo "================================================="
echo "  Configuration successfully saved!"
echo "  - Config File: $DOOR_DEVICE_JSON"
echo "  - Internal Settings: $INTERNAL_SETTINGS_JSON"
echo "================================================="
