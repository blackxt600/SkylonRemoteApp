# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based HTTP server for controlling Kettler exercise equipment (bikes, treadmills, crosstrainers) via Bluetooth. It wraps the `kdri` library (Kettler Device Rust Interface) and exposes a REST API using Actix-web for reading device metrics and controlling resistance/speed.

## Build and Run Commands

```bash
# Build the project
cargo build

# Run the server (requires bluetooth device at /dev/rfcomm0)
cargo run

# Build in release mode
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check
```

## Architecture

### Three-Layer Design

1. **HTTP Server Layer** (`src/main.rs`)
   - Actix-web REST API with two endpoints:
     - `GET /status` - Returns current speed, pulse, and resistance
     - `POST /resistance/{level}` - Sets resistance level (0-16)
   - Binds to `0.0.0.0:8080`
   - Uses Arc-wrapped BikeController for shared state

2. **Controller Layer** (`src/bike_controller.rs`)
   - `BikeController` manages async access to the Bluetooth connection
   - Uses tokio Mutex for thread-safe access to blocking kdri library
   - Runs a background polling task to periodically update device data
   - Bridges async/await (tokio) with blocking Bluetooth I/O via `spawn_blocking`

3. **Bluetooth Protocol Layer** (`src/lib.rs`)
   - `kdri` library from https://github.com/kaegi/kdri
   - Implements the Kettler proprietary protocol over RFCOMM Bluetooth
   - Uses mio event loop for non-blocking I/O
   - Handles CRC checksums (src/crc.rs), packet escaping, and device-specific value polling
   - `KettlerConnection` exposes getters for speed, pulse, RPM, distance, etc.

### Key Async/Blocking Boundary

The kdri library is synchronous/blocking, while the HTTP server is async. The `BikeController` uses `tokio::task::spawn_blocking` to run blocking kdri calls in dedicated threads without blocking the async runtime.

### Device Connection Flow

1. Server starts → `BikeController::new()` is called
2. Scans for Kettler devices via Bluetooth (blocking)
3. Connects to last discovered device at `/dev/rfcomm0`
4. Starts background polling task (default 1 second intervals)
5. HTTP endpoints become available

## Important Implementation Details

### Resistance Control

The current implementation in `bike_controller.rs:89` uses `set_incline()` as a placeholder for resistance control. The kdri library provides `set_brake_level()` for bikes/crosstrainers and `set_incline()` for treadmills. You may need to adjust this based on the actual device type.

### Value Scaling

Speed values from kdri are in 0.1 km/h units (e.g., 105 = 10.5 km/h). The BikeController divides by 10.0 to convert to km/h for the API response.

### Error Handling

The project uses `anyhow::Result` for error handling. Bluetooth errors and connection failures will cause the server to fail at startup.

## Device Compatibility

Supports Kettler devices with names starting with:
- TOUR, RACER, ERGO, RECUMBENT, UNIX, SKYLON, RUN, TRACK

The library auto-detects device type and polls appropriate values (e.g., RPM for bikes, speed for treadmills).

## Files

- `src/main.rs` - HTTP server and API endpoints
- `src/bike_controller.rs` - Async controller wrapping kdri connection
- `src/lib.rs` - kdri library (Kettler protocol implementation)
- `src/crc.rs` - CRC16 checksum calculation
- `src/main-example.rs` - Example CLI for direct device interaction
- `Cargo.toml` - Dependencies: actix-web, tokio, kdri (git), serde, anyhow

## Known Issues

- Hardcoded device path `/dev/rfcomm0` in main.rs
- No graceful shutdown handling for BikeController polling task
- Resistance control uses incline instead of proper brake_level API
- No device discovery endpoint (always connects to last scanned device)
