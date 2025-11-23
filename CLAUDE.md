# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust-based HTTP server for controlling Kettler elliptical trainers via Bluetooth. The project includes a modern web interface optimized for 11" tablets and supports custom training programs with automatic power adjustments. The server wraps the `kdri` library (Kettler Device Rust Interface) and exposes a REST API using Actix-web.

**Note**: This project uses Rust edition 2024 in Cargo.toml, which requires a recent Rust toolchain.

## Build and Run Commands

### Production (Raspberry Pi with Bluetooth)

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

### Development (macOS/Linux without Bluetooth - Mock Mode)

The project includes a mock mode for development on systems without Bluetooth hardware:

```bash
# Build in mock mode (simulates bike data)
cargo build --release --no-default-features --features mock

# Run in mock mode
cargo run --release --no-default-features --features mock
```

**Mock mode features:**
- Simulates realistic RPM variations (40-80 RPM with sinusoidal patterns)
- Calculates speed based on RPM (~10.8 km/h at 60 RPM)
- Full API support (status, power control, training programs)
- No Bluetooth hardware required
- Ideal for frontend development on macOS

See `MOCK_MODE.md` for detailed documentation on mock mode development.

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

- Main interface: `static/index.html` - Modern glassmorphism design with **3-column layout**:
  - **Left column (160px)**: Programs sidebar
    - 10 program buttons: Plat (manual), Escalier, Vallée, Collines, Montagne, Col Alpin, Intervalle, Pyramide, Sur mesure, Jeu
    - Vertical flex distribution with `justify-content: space-evenly`
    - Buttons use `flex: 1` for equal height distribution
  - **Center column**: Histogram/game display with mode-specific controls
    - Difficulty control for standard programs
    - Custom program editor with random generator (🎲 button)
    - Manual power control for Plat mode
    - Space Runner game canvas for Jeu mode
    - Visual histogram with real-time progression
  - **Right column (240px)**: Control panel
    - Date/time display
    - Power display (3em font, gradient effect)
    - Timer with auto-start/pause based on RPM
    - Enlarged playback controls (Pause/Play/Reset with flex: 1)
  - **Bottom section (218px)**: RPM graph with target line and color-coded tracking
- Container: 99vh height for better screen utilization
- Program manager: `static/programs.html` - Create custom training programs
- **Game Mode**: Space Runner - RPM-controlled arcade game
  - Ship position controlled by pedaling speed (0-100 RPM)
  - Asteroids (obstacles) and stars (collectibles)
  - Real-time score and distance tracking
  - Particle effects and game over screen
- Optimized for 11" tablets in landscape mode
- System shutdown/reboot buttons (bottom-right) require sudo passwordless configuration

### Training Programs

Ten modes available:
- **Plat** (Manual mode): Direct power control with flat histogram visualization
- **Escalier** (Stepped): Progressive power levels (formerly "Plat")
- **Vallée, Collines, Montagne, Col Alpin, Intervalle, Pyramide**: Standard 30-minute programs
- **Sur mesure** (Custom): Editable program with random generator
  - 30 intervals of 1 minute each
  - Random generator: 25-30-35W warm-up, then 50-110W random
  - Persistent in localStorage
- **Jeu** (Game): Space Runner arcade mode controlled by RPM
- Programs stored in `BikeController.programs` (HashMap) for backend execution
- Frontend programs stored in localStorage for customization
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
├── main.rs                  # HTTP server and API endpoints (with conditional compilation)
├── bike_controller.rs       # Real Bluetooth controller (#[cfg(feature = "real-bluetooth")])
├── mock_bike_controller.rs  # Mock simulation controller (#[cfg(feature = "mock")])
├── training_program.rs      # Program data structures and execution state (shared)
└── main-example.rs          # Example CLI for direct device interaction

static/
├── index.html          # Main web interface (tablet-optimized)
└── programs.html       # Program creation/management interface

autostart/              # Systemd auto-start configuration

MOCK_MODE.md            # Documentation for mock mode development
```

### Conditional Compilation

The project uses Rust feature flags to support both real Bluetooth and mock modes:

- `Cargo.toml` defines features: `real-bluetooth` (default) and `mock`
- `main.rs` conditionally imports the appropriate controller module
- `bike_controller.rs` only compiles with `real-bluetooth` feature
- `mock_bike_controller.rs` only compiles with `mock` feature
- Both controllers implement identical APIs for seamless switching

## Known Limitations

- Bluetooth device scanning may fail if device not paired/trusted
- Requires `/dev/rfcomm` or native Bluetooth stack access
- System shutdown/reboot endpoints require sudo configuration
- Frontend program customization stored in localStorage (not persisted server-side)
- Always make comments and explanations in English
