# Mock Mode - Development on macOS

This document explains how to use the mock mode for developing the SkylonRemoteApp on macOS (or any system without Bluetooth hardware).

## Overview

The project includes a **mock mode** that simulates the Kettler elliptical bike without requiring actual Bluetooth hardware. This allows developers to:

- Work on the web interface on macOS
- Test API endpoints without physical hardware
- Develop and debug features locally
- Run the server in CI/CD environments

## How Mock Mode Works

### Simulated Data

The mock controller (`src/mock_bike_controller.rs`) provides:

- **Realistic RPM simulation**: Varies between 40-80 RPM with sinusoidal patterns
- **Speed calculation**: Based on RPM (~0.18 km/h per RPM)
- **Power control**: Full support for setting power levels (25-400W)
- **Training programs**: Complete program execution support
- **Always connected**: No Bluetooth connection failures

### What's Different from Real Mode

| Feature | Real Bluetooth | Mock Mode |
|---------|---------------|-----------|
| Bluetooth connection | Required | Not used |
| Device scanning | Scans for Kettler devices | Skipped |
| RPM/Speed data | From actual device | Simulated with realistic patterns |
| Power control | Sends to device | Logged only |
| Reconnection logic | Full retry mechanism | Not needed |
| Training programs | Fully supported | Fully supported |

## Building and Running

### Compile in Mock Mode (macOS)

```bash
# Build in mock mode
cargo build --release --no-default-features --features mock

# Run in mock mode
cargo run --release --no-default-features --features mock
```

### Compile in Real Bluetooth Mode (Raspberry Pi)

```bash
# Build with real Bluetooth (default)
cargo build --release

# Or explicitly
cargo build --release --features real-bluetooth
```

## Usage Examples

### Start Mock Server

```bash
cd SkylonRemoteApp
cargo run --release --no-default-features --features mock
```

Output:
```
🚀 Démarrage du serveur elliptique...
🔧 MODE: SIMULATION (mock) - Pas de connexion Bluetooth réelle
🔧 Mode MOCK: Simulation du contrôleur de vélo
   Pas de connexion Bluetooth réelle
✅ Contrôleur mock initialisé avec succès
🔄 Mode MOCK: Simulation en cours (pas de polling Bluetooth)
🌐 Serveur web démarré sur http://0.0.0.0:8080
```

### Test API Endpoints

```bash
# Get status (RPM/Speed will vary realistically)
curl http://localhost:8080/status

# Set power level
curl -X POST http://localhost:8080/power/150

# Get current power
curl http://localhost:8080/power
```

### Access Web Interface

Open your browser to `http://localhost:8080` and you'll see:
- Real-time simulated RPM/speed data
- Working power controls
- Training programs (fully functional)
- All UI features

## Development Workflow

### Recommended Workflow for macOS Developers

1. **Frontend Development**:
   - Run server in mock mode on macOS
   - Edit HTML/CSS/JS in `static/` directory
   - Test UI changes immediately with live reload

2. **Backend Development**:
   - Make changes to API endpoints in `src/main.rs`
   - Test with mock controller
   - Ensure changes work with both `BikeController` and `MockBikeController`

3. **Testing on Real Hardware**:
   - Deploy to Raspberry Pi
   - Build with `--features real-bluetooth`
   - Test with actual Kettler device

### Code Structure

```
src/
├── main.rs                    # Conditionally imports correct controller
├── bike_controller.rs         # Real Bluetooth implementation (#[cfg(feature = "real-bluetooth")])
├── mock_bike_controller.rs    # Mock simulation (#[cfg(feature = "mock")])
└── training_program.rs        # Shared by both modes
```

The `main.rs` uses conditional compilation to import the correct controller:

```rust
#[cfg(feature = "real-bluetooth")]
mod bike_controller;

#[cfg(feature = "mock")]
mod mock_bike_controller;

#[cfg(feature = "mock")]
use mock_bike_controller as bike_controller;
```

## Feature Flags in Cargo.toml

```toml
[features]
default = ["real-bluetooth"]
real-bluetooth = ["kdri"]
mock = []
```

- **default**: Uses `real-bluetooth` (for Raspberry Pi)
- **real-bluetooth**: Includes `kdri` dependency and Bluetooth code
- **mock**: Excludes `kdri`, uses simulation

## Customizing Mock Behavior

To modify the simulation, edit `src/mock_bike_controller.rs`:

```rust
// Change RPM pattern
let base_rpm = 60;  // Average RPM
let rpm_variation = ((elapsed as f32 * 0.3).sin() * 10.0) as i16;  // ±10 RPM variation

// Change speed calculation
let speed = (rpm as f32) * 0.18;  // 0.18 km/h per RPM
```

## Troubleshooting

### Error: "cannot find type `BtSocket`"

You're trying to build without specifying `--features mock` on macOS.

**Solution**: Always use `--no-default-features --features mock` on macOS.

### Warning: "method `remaining_time` is never used"

This is harmless - it's a utility method in `training_program.rs` that may be used in the future.

### Server doesn't respond

Check that port 8080 is not already in use:
```bash
lsof -i :8080
```

## Deploying to Raspberry Pi

When deploying to production on Raspberry Pi:

1. **Use default features** (builds with real Bluetooth):
   ```bash
   cargo build --release
   ```

2. **Or explicitly specify**:
   ```bash
   cargo build --release --features real-bluetooth
   ```

3. **The systemd service** (in `autostart/`) uses the default build, which is real Bluetooth mode.

## Benefits of Mock Mode

✅ **Develop on macOS** without Raspberry Pi
✅ **No Bluetooth hardware needed** for UI development
✅ **Faster iteration** - compile and test locally
✅ **Realistic simulation** - RPM and speed vary naturally
✅ **Full feature support** - training programs work identically
✅ **CI/CD friendly** - can run tests without hardware

## Limitations

⚠️ Cannot test:
- Actual Bluetooth connection/reconnection logic
- Real device communication issues
- Hardware-specific edge cases
- Bluetooth pairing problems

For these scenarios, testing on Raspberry Pi with real hardware is required.

---

**Happy coding!** 🚀
