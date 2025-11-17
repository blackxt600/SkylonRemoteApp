# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust-based HTTP server for controlling Kettler elliptical trainers via Bluetooth. The project includes a modern web interface optimized for 11" tablets and supports custom training programs with automatic power adjustments. The server wraps the `kdri` library (Kettler Device Rust Interface) and exposes a REST API using Actix-web.

**Note**: This project uses Rust edition 2024 in Cargo.toml, which requires a recent Rust toolchain.

## Build and Run Commands

```bash
# Build the project
cargo build

# Run the server (requires Bluetooth device)
cargo run

# Build in release mode (recommended for deployment)
cargo build --release

# Run in release mode
cargo run --release

# Run tests
cargo test

# Check code without building
cargo check
```

## Architecture

### Three-Layer Design

1. **HTTP Server Layer** (`src/main.rs`)
   - Actix-web REST API with multiple endpoint groups:
     - Status: `GET /status`, `GET /power`
     - Power control: `POST /power/{level}` (25-400W)
     - Program management: CRUD operations on training programs
     - System control: `POST /system/shutdown`, `POST /system/reboot`
   - Serves static files from `./static` directory
   - Binds to `0.0.0.0:8080`

2. **Controller Layer** (`src/bike_controller.rs`)
   - `BikeController` manages async access to Bluetooth connection
   - Handles automatic reconnection with exponential backoff (up to 5 attempts)
   - Background polling task updates device data every second
   - Training program execution engine that automatically adjusts power levels
   - Bridges async/await (tokio) with blocking kdri library via `spawn_blocking`
   - Thread-safe state management using Arc<Mutex<T>>

3. **Training Program Layer** (`src/training_program.rs`)
   - `TrainingProgram` - Defines workout programs with multiple intervals
   - `TrainingInterval` - Individual workout segments (duration + power target)
   - `ProgramExecutionState` - Tracks active program progress
   - Validation ensures intervals have duration > 0 and power 25-400W

### Key Async/Blocking Boundary

The kdri library is synchronous/blocking, while the HTTP server is async. The `BikeController` uses `tokio::task::spawn_blocking` to run blocking kdri calls in dedicated threads without blocking the async runtime.

### Device Connection Flow

1. Server starts → `BikeController::new()` initializes without blocking
2. Background task attempts connection (up to 3 retries with 3s delays)
3. Scans for Kettler devices via Bluetooth
4. Connects to last discovered device
5. If initial connection fails, retries every 30 seconds
6. HTTP endpoints available immediately (return disconnected state if not connected)
7. Background polling updates device data every second

### Training Program Execution

1. Program created/loaded via REST API or localStorage (web UI)
2. `POST /program/{id}/start` initiates execution
3. Background loop (1-second interval) advances program state
4. Power level automatically adjusted when transitioning between intervals
5. Program stops automatically when all intervals complete
6. Frontend displays real-time progress with histogram visualization

## Important Implementation Details

### Power Control

- Valid range: **25-400W** (enforced in both frontend and backend)
- Backend validation in `bike_controller.rs:233` (`set_power()`)
- Uses `kdri`'s `set_power()` method (not incline or brake level)
- Power changes are logged to console

### Value Scaling

Speed values from kdri are in 0.1 km/h units (e.g., 105 = 10.5 km/h). The BikeController divides by 10.0 to convert to km/h for the API response.

### Reconnection Strategy

The controller implements robust reconnection handling:
- Maximum 5 consecutive reconnection attempts with exponential backoff
- Clears old connection before attempting new one
- Counter resets after successful reconnection
- Server continues running even if Bluetooth unavailable

### Error Handling

Uses `anyhow::Result` for error handling. Bluetooth connection failures are non-fatal - the server starts and continues retrying in the background.

### Web Interface

- Main interface: `static/index.html` - Modern glassmorphism design with:
  - Real-time RPM tracking with color-coded graphing
  - 9 predefined training programs (editable via localStorage)
  - Manual power control with adjustable step sizes (5W, 10W, 25W, 50W)
  - Auto-start/pause timer based on RPM threshold
  - Program statistics summary at completion
- Program manager: `static/programs.html` - Create custom training programs
- Optimized for 11" tablets in landscape mode
- System shutdown/reboot buttons require sudo passwordless configuration for the user running the server

### Training Programs

Nine predefined programs included (30 minutes each):
- Plat, Vallée, Collines, Montagne, Col Alpin, Intervalle, Pyramide, Changement, Altitude
- Programs stored in `BikeController.programs` (HashMap)
- Frontend can customize programs via localStorage
- Each program has multiple intervals with power targets and durations

## Device Compatibility

Supports Kettler devices with names starting with:
- TOUR, RACER, ERGO, RECUMBENT, UNIX, SKYLON, RUN, TRACK

The kdri library auto-detects device type and polls appropriate values (RPM, power, speed).

## API Endpoints

### Status & Control
- `GET /status` - Returns {speed, rpm, power, connected}
- `GET /power` - Returns current power level
- `POST /power/{level}` - Set power (25-400W)

### Training Programs
- `POST /program` - Create new program
- `GET /programs` - List all programs
- `GET /program/{id}` - Get specific program
- `PUT /program/{id}` - Update program
- `DELETE /program/{id}` - Delete program
- `POST /program/{id}/start` - Start program execution
- `POST /program/stop` - Stop active program
- `GET /program/active` - Get active program state

### System Control
- `POST /system/shutdown` - Shutdown Raspberry Pi (requires sudo config - see SYSTEM_SHUTDOWN_REBOOT.md)
- `POST /system/reboot` - Reboot Raspberry Pi (requires sudo config - see SYSTEM_SHUTDOWN_REBOOT.md)

## Deployment

For systemd auto-start configuration, see:
- `autostart/startup-command.service` - Systemd service file
- `autostart/launch_terminal.sh` - Launch script
- `autostart/README_installation.md` - Installation instructions
- `autostart/GESTION_LOGS.md` - Log management for Raspberry Pi (important to prevent disk space issues)

For shutdown/reboot functionality, see `SYSTEM_SHUTDOWN_REBOOT.md` for detailed sudo configuration instructions to allow passwordless execution of `/sbin/shutdown` and `/sbin/reboot` for the user running the server.

### Log Management (Important for Raspberry Pi)

The application outputs logs to stdout/stderr which are captured by systemd's journald. Without configuration, logs can grow to 200-300 MB and fill limited disk space on Raspberry Pi.

**Recommended configuration:**
- Use `autostart/journald-limit.conf` to limit journal size to 50 MB
- Set up weekly log cleanup with `autostart/cleanup-logs.sh`
- See `autostart/GESTION_LOGS.md` for complete setup instructions

## Versioning

Project follows Semantic Versioning. Update both `VERSION` file and `CHANGELOG.md` when releasing. See README.md for detailed versioning workflow.

## Files Structure

```
src/
├── main.rs              # HTTP server and API endpoints
├── bike_controller.rs   # Async controller with reconnection logic
├── training_program.rs  # Program data structures and execution state
└── main-example.rs      # Example CLI for direct device interaction

static/
├── index.html          # Main web interface (tablet-optimized)
└── programs.html       # Program creation/management interface

autostart/              # Systemd auto-start configuration
```

## Known Limitations

- Bluetooth device scanning may fail if device not paired/trusted
- Requires `/dev/rfcomm` or native Bluetooth stack access
- System shutdown/reboot endpoints require sudo configuration
- Frontend program customization stored in localStorage (not persisted server-side)
- Always make comments and explanations in English
