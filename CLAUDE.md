# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust-based HTTP server for controlling Kettler elliptical trainers via Bluetooth. The project includes a modern web interface optimized for 11" tablets and supports custom training programs with automatic power adjustments. The server wraps the `kdri` library (Kettler Device Rust Interface) and exposes a REST API using Actix-web.

**Important**: This project uses Rust edition 2024 in Cargo.toml, which requires Rust 1.85.0 or newer. Update your toolchain with `rustup update` if you encounter edition errors.

## Quick Start

**On Raspberry Pi (with Bluetooth):**
```bash
cargo run --release
```

**On macOS/Linux (development without Bluetooth):**
```bash
cargo run --release --no-default-features --features mock
```

Server will be available at `http://0.0.0.0:8080`

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

# Quick run with convenience script
./run-mock.sh
```

**Mock mode features:**
- Simulates realistic RPM variations (40-80 RPM with sinusoidal patterns)
- Calculates speed based on RPM (~10.8 km/h at 60 RPM)
- Full API support (status, power control, training programs)
- No Bluetooth hardware required
- Ideal for frontend development on macOS

See `MOCK_MODE.md` for detailed documentation on mock mode development.

## Dependencies

### For Production (Raspberry Pi)

```bash
# Install Bluetooth development libraries
sudo apt-get update
sudo apt-get install bluez libbluetooth-dev

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ensure Rust is up to date (edition 2024 requires Rust 1.85.0+)
rustup update
```

### For Development (macOS)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update to latest Rust (edition 2024 requires Rust 1.85.0+)
rustup update

# No Bluetooth libraries needed for mock mode
```

## Testing

Currently, the project has minimal automated tests. Testing is primarily done through:

1. **Manual testing with real hardware** on Raspberry Pi
2. **Mock mode testing** for frontend development on macOS
3. **API testing** with curl or browser

To run any existing tests:
```bash
cargo test
```

**Note:** Most functionality requires either real Bluetooth hardware or mock mode, so unit tests are limited. Integration testing is done manually through the web interface.

## Useful Development Commands

```bash
# Check for compilation errors without building
cargo check

# Check with specific features
cargo check --no-default-features --features mock

# Build in debug mode (faster compilation, slower runtime)
cargo build

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Clean build artifacts
cargo clean

# View dependency tree
cargo tree

# Update dependencies
cargo update
```

### Testing the API with curl

```bash
# Get status
curl http://localhost:8080/status

# Set power to 100W
curl -X POST http://localhost:8080/power/100

# Get current power
curl http://localhost:8080/power

# List programs
curl http://localhost:8080/programs

# Get active program
curl http://localhost:8080/program/active
```

### Direct Device Testing (main-example.rs)

For low-level Bluetooth testing without the HTTP server, use the example CLI:

```bash
# This directly interfaces with the Kettler device
cargo run --bin main-example --features real-bluetooth
```

This is useful for:
- Testing Bluetooth connection issues
- Debugging kdri library integration
- Quick device functionality checks
- Understanding the raw device protocol

**Note:** Only works on Raspberry Pi with real Bluetooth hardware.

## Logging and Debugging

### Understanding Server Logs

When the server starts, you'll see different messages depending on the mode:

**Real Bluetooth Mode (Raspberry Pi):**
```
🚀 Démarrage du serveur elliptique...
🔍 Recherche d'appareils Kettler...
📱 Appareil trouvé : SKYLON
✅ Connecté avec succès !
🌐 Serveur web démarré sur http://0.0.0.0:8080
```

**Mock Mode (macOS/Linux):**
```
🚀 Démarrage du serveur elliptique...
🔧 MODE: SIMULATION (mock) - Pas de connexion Bluetooth réelle
🔧 Mode MOCK: Simulation du contrôleur de vélo
   Pas de connexion Bluetooth réelle
✅ Contrôleur mock initialisé avec succès
🔄 Mode MOCK: Simulation en cours (pas de polling Bluetooth)
🌐 Serveur web démarré sur http://0.0.0.0:8080
```

### Connection Issues

If you see repeated connection attempts:
```
⚠️  Tentative 1/3 échouée : ...
🔄 Nouvelle tentative dans 3 secondes...
⚠️  Impossible de se connecter pour le moment.
   Le serveur continue de fonctionner. Réessai automatique toutes les 30 secondes...
🔄 Tentative de connexion automatique...
```

This means:
- Bluetooth device not found or not paired
- Device out of range
- Bluetooth adapter issues

The server continues running and will auto-connect when device becomes available.

### Viewing Logs

**When running directly:**
- Logs appear in the terminal where you ran `cargo run`

**When running as systemd service:**
```bash
# View recent logs
sudo journalctl -u startup-command.service -n 50

# Follow logs in real-time
sudo journalctl -u startup-command.service -f

# Logs since last hour
sudo journalctl -u startup-command.service --since "1 hour ago"

# Check log disk usage
journalctl --disk-usage
```

### Debug Mode

For more verbose output during development, you can modify the log level:

```bash
# Run with Rust backtrace for better error messages
RUST_BACKTRACE=1 cargo run --release

# Run with full backtrace
RUST_BACKTRACE=full cargo run --release
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

The kdri library is synchronous/blocking, while the HTTP server is async. This is a critical architectural constraint.

**How it works:**
- All kdri operations (scan, connect, get/set values) are synchronous and block the calling thread
- The HTTP server uses tokio async runtime and cannot block
- Solution: `BikeController` wraps all kdri calls in `tokio::task::spawn_blocking`
- This runs blocking operations in a dedicated thread pool without blocking async tasks

**Example pattern in `BikeController`:**
```rust
pub async fn set_power(&self, level: u16) -> Result<()> {
    let connection = Arc::clone(&self.connection);

    // Move into blocking thread pool
    tokio::task::spawn_blocking(move || {
        let mut conn = connection.lock().unwrap();
        if let Some(ref mut c) = *conn {
            c.set_power(level)
        } else {
            bail!("Not connected")
        }
    }).await?  // Await the JoinHandle, then unwrap the Result
}
```

**Important when modifying code:**
- NEVER call kdri methods directly from async functions
- ALWAYS wrap kdri calls in `spawn_blocking`
- Remember to clone `Arc<Mutex<T>>` before moving into the blocking closure
- The pattern is: async function → spawn_blocking → lock mutex → call kdri → return result

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
- Backend validation in `BikeController::set_power()` method
- Uses `kdri`'s `set_power()` method (not incline or brake level)
- Power changes are logged to console
- Invalid values return `anyhow::Error` with message "Niveau de puissance hors plage (25-400)"
- Power setting is async and uses `spawn_blocking` to handle the synchronous kdri library

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

## Development Workflow Best Practices

### Frontend Development
1. Run server in mock mode on macOS: `./run-mock.sh`
2. Edit HTML/CSS/JS files in `static/` directory
3. Refresh browser to see changes (hard refresh if needed: Cmd+Shift+R)
4. Test with browser DevTools console open to catch errors
5. Test on both desktop and tablet viewport sizes

### Backend Development
1. Make changes to Rust code in `src/`
2. Run `cargo check` for quick syntax validation
3. Test in mock mode first: `cargo run --no-default-features --features mock`
4. Deploy to Raspberry Pi for real hardware testing
5. Check server logs for errors and connection status

### Making Changes to BikeController
- Remember the async/blocking boundary (see Architecture section)
- Always use `spawn_blocking` for kdri calls
- Test with both real and mock controllers
- Mock controller should mirror the real controller's API exactly

### Adding New API Endpoints
1. Add route handler in `src/main.rs` with appropriate macro (`#[get]`, `#[post]`, etc.)
2. Extract data from `web::Data<Arc<BikeController>>`
3. Call controller methods (which handle async/blocking properly)
4. Return JSON response or error
5. Register endpoint in `HttpServer::new()` app configuration
6. Test with curl before testing in browser

### Modifying the Web Interface
- Main interface is `static/index.html` (single-file design)
- Styles are embedded in `<style>` tag
- JavaScript is embedded in `<script>` tag
- 3-column layout: programs (left), histogram/game (center), controls (right)
- RPM graph is in the bottom section
- Use CSS variables for consistent theming
- Test on actual 11" tablet if possible

## Versioning

Project follows Semantic Versioning (MAJOR.MINOR.PATCH). When releasing:

1. Update the `VERSION` file with new version number
2. Update `CHANGELOG.md` with changes in this release
3. Commit both files together
4. Create git tag: `git tag -a v1.2.3 -m "Version 1.2.3"`
5. Push with tags: `git push origin main --tags`

See README.md for detailed versioning workflow and examples.

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

## Troubleshooting

### Build Issues

**Error: "edition2024" is unstable**
- Cause: Rust version too old
- Solution: Run `rustup update` to get Rust 1.85.0 or newer

**Error: "cannot find type `BtSocket`" on macOS**
- Cause: Trying to build with real Bluetooth features on macOS
- Solution: Use mock mode: `cargo build --no-default-features --features mock`

**Error: "failed to find package `kdri`"**
- Cause: Git dependency not accessible or network issue
- Solution: Check internet connection; kdri is fetched from GitHub

### Runtime Issues

**Server starts but shows "⚠️ Impossible de se connecter"**
- Cause: Bluetooth device not found or not paired
- Solution:
  - Ensure Kettler device is powered on and in pairing mode
  - Check Bluetooth pairing with `bluetoothctl`
  - Verify device is within range

**API returns `connected: false`**
- Cause: Bluetooth connection failed
- Solution: Check server logs for specific error messages; server continues running and retries every 30 seconds

**System shutdown/reboot buttons don't work**
- Cause: Sudo not configured for passwordless shutdown/reboot
- Solution: Follow instructions in `SYSTEM_SHUTDOWN_REBOOT.md`

**Logs filling up disk space on Raspberry Pi**
- Cause: Systemd journal not configured with size limits
- Solution: Follow instructions in `autostart/GESTION_LOGS.md` to limit journal to 50 MB

**Port 8080 already in use**
- Cause: Another instance or application using port 8080
- Solution: `lsof -i :8080` to find process, kill it or change port in `main.rs`

### Development Issues

**Frontend changes not visible**
- Cause: Browser caching static files
- Solution: Hard refresh (Ctrl+Shift+R or Cmd+Shift+R) or disable cache in DevTools

**Mock mode RPM stays at 0**
- Cause: Mock controller simulation might not be updating
- Solution: Check server logs; mock controller should log "Mode MOCK: Simulation en cours"

**Training program not advancing**
- Cause: Program loop not started or program state not updating
- Solution: Check server logs for program execution messages; ensure RPM > 0 for timer to advance

## Known Limitations

- Bluetooth device scanning may fail if device not paired/trusted beforehand
- Requires `/dev/rfcomm` or native Bluetooth stack access on Linux
- System shutdown/reboot endpoints require sudo configuration (see `SYSTEM_SHUTDOWN_REBOOT.md`)
- Frontend program customization stored in localStorage (not persisted server-side)
- Cannot run real Bluetooth mode on macOS (use mock mode for development)
- Training programs assume 1-second update interval; system lag may affect timing accuracy
- No authentication/authorization on API endpoints (designed for trusted local network only)
- Always make comments and explanations in English
