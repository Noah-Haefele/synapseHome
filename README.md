> [!WARNING]
> **Work in Progress — not production-ready.**
> The codebase is under active development. Expect incomplete features, breaking changes, and missing documentation. A first stable release is being worked toward.

---

# synapseHome

**synapsed** is the daemon at the core of synapseHome — a fully self-hosted, open-source door intercom system built for residential and small commercial use.

The goal is a fully self-hosted system that operates entirely on your local network, without cloud services, subscriptions, or an internet connection during normal operation. Communication runs over ethernet, audio is transmitted peer-to-peer in high quality with acoustic echo cancellation for clean full-duplex conversations, and the whole thing is designed from the ground up to be modular and extensible. Planned hardware support includes cameras with night vision for outdoor stations, door release control, and a flexible device model that can grow with your installation.

Every intercom station runs `synapsed`. Stations discover each other through a shared MQTT broker on your LAN. When one station initiates a call, the daemon handles signalling, establishes a direct UDP audio stream between the two endpoints, and manages the full call lifecycle — from ring to hangup. The gRPC API it exposes means any client can integrate with it: touch UI, mobile app, web frontend, or your own tooling.

---

## Getting Started

> [!NOTE]
> The first build compiles the Rust daemon and the C++-based UI from source. Depending on your hardware this can take **up to two hours** — on a Raspberry Pi noticeably longer than on a desktop machine.

### Step 1 — Run the installation script (supports most Linux distributions)
> [!WARNING]
> Running this can take **up to two hours** depending on your hardware and network connection (Ethernet vs. Wi-Fi). On a typical desktop PC, installation should take around 30 minutes.

From the repo root:
```bash
chmod +x install.sh
./install.sh
```
This will install all required dependencies and compilers and build the application.

### Step 2 — Set up an MQTT broker

synapseHome requires an MQTT broker that every station on the network can reach. All call signalling runs through it.

**Option A — Local broker (recommended):** run [Mosquitto](https://mosquitto.org/) on any machine in your LAN, or directly on one of the intercom stations:

```bash
sudo apt-get install mosquitto mosquitto-clients
```

**Option B — Public test broker:** `test.mosquitto.org:1883` (provided by the Eclipse Foundation)
> [!CAUTION]
> The public test broker is shared with other users and is not suitable for private or production deployments. Use a dedicated broker for real installations.

> A setup script that installs and pre-configures a local Mosquitto instance is coming with the first stable release.

### Step 3 — Configure

Edit the two config files before running anything:

**`config/mqtt.json`** — point every station at your broker:
```json
{
  "clientId": "station-1f",
  "brokerIp": "192.168.1.10",
  "brokerPort": 1883
}
```

Use a unique `clientId` per station. The broker IP must be reachable from all stations.

**`config/devices.json`** — define the station topology:
```json
{
  "devices": [
    { "deviceName": "1st Floor", "deviceShortName": "1", "deviceId": 1 },
    { "deviceName": "2nd Floor", "deviceShortName": "2", "deviceId": 2 },
    { "deviceName": "3rd Floor", "deviceShortName": "3", "deviceId": 3 }
  ]
}
```

Every station in the network must carry the same `devices.json`. The daemon reads this at startup to build the call routing table.

### Step 4 — Start the daemon

```bash
cargo run --release --bin touch_device
```

This compiles and starts `synapsed`. The gRPC server comes up on port `50051` and the station begins listening for MQTT events.

### Step 5 — Launch the UI

```bash
./build/sHUiBridge
```

The UI connects to the local daemon via gRPC. The station is ready.

---

## How it works

### Device model

Every station is identified by a numeric device ID and a human-readable name (e.g. *"1st Floor"*, *"2nd Floor"*). The device list is defined once in `config/devices.json` and shared across all stations. Each station knows its own ID and can therefore address any other device by ID.

### Signalling

Signalling runs over MQTT. Each station subscribes to a topic derived from its device ID and listens for call events published by other stations. The daemon wraps `rumqttc` for MQTT transport and handles connection management internally.

### Audio

Once a call is established, audio is streamed peer-to-peer over UDP — no server relay involved. The audio pipeline uses `cpal` for device I/O and `rodio` for playback. Acoustic echo cancellation is applied via `webrtc-audio-processing` so that full-duplex conversations work cleanly without feedback loops.

### API surface

`synapsed` exposes a gRPC server on port **50051**. The API is defined in `proto/` and covers three domains:

| Proto file | Services |
|---|---|
| `call_api.proto` | `CallSignals` (streaming event subscription), `CallActions` (initiate, accept, end), `CallHelpers` (get call label) |
| `settings_api.proto` | `System` (device list, location ID, IP), `Display` (brightness, display timeout), `Audio` (sink/source selection), `Ringtone` (ringtone selection) |
| `pref_api.proto` | Preferences: call ID list, icon paths, display names, device model |

Any client that can speak gRPC can integrate with it — the reference client is a touch-screen UI running on the same device, but nothing prevents a mobile app or a web frontend from connecting to the same socket.

---

## Repository layout

```
synapsed/
├── src/
│   ├── core/
│   │   ├── act/
│   │   │   ├── audio/        # Audio I/O, AEC, device enumeration
│   │   │   └── call/         # Call lifecycle (setup, handler, MQTT event handler)
│   │   ├── api/              # gRPC service implementations
│   │   ├── display/          # Display brightness management
│   │   └── state/            # Device manager, ringtone manager
│   ├── networking/
│   │   ├── audio/            # UDP audio sender / receiver
│   │   └── mqtt/             # MQTT client and config
│   └── platform/
│       └── linux/            # Linux-specific display controller
├── proto/                    # Protobuf definitions (source of truth for the API)
└── config/
    ├── devices.json          # Device list
    └── mqtt.json             # MQTT broker connection settings
```

---

## What's still coming

This project is in early development. The following areas are actively being worked on and will land before a stable release:

- **Systemd service files** — `.service` unit files for `synapsed` to run as a proper system daemon with auto-restart and dependency ordering.
- **Encryption** — Add encryption for audio streams and MQTT communication to protect communications between stations.
- **Video cameras** — each door device can have a camera connected to it which feeds its video stream to the touch devices inside the building.
- **Utility scripts** — helpers for managing the device list, testing MQTT connectivity, and diagnosing audio routing issues.
- **Video guides** — walkthrough videos covering hardware setup, network configuration, and getting the system running end-to-end.
- **Documentation** — network topology guide, full API documentation generated from the proto files, and more.

---

## License

See [LICENSE](LICENSE).
