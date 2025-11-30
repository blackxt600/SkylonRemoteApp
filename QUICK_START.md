# Quick Start Guide

## TL;DR

### On macOS (Mock Mode - No Bluetooth)
```bash
cargo run --release --no-default-features --features mock
```
Or use the helper script:
```bash
./run-mock.sh
```

### On Raspberry Pi (Real Bluetooth)
```bash
cargo run --release
```

## What is Mock Mode?

Mock mode simulates a Kettler elliptical bike without requiring Bluetooth hardware. It's perfect for:
- 🖥 Developing on macOS
- 🎨 Working on the web interface
- 🧪 Testing API endpoints
- 🚀 Fast iteration without hardware

## Mock Mode Features

✅ **Realistic simulation:**
- RPM varies between 40-80 (sinusoidal pattern)
- Speed calculated from RPM (~0.18 km/h per RPM)
- Power control fully functional (25-400W)
- Training programs work identically

✅ **Full API support:**
- All endpoints work (`/status`, `/power`, `/programs`, etc.)
- Same behavior as real hardware
- No Bluetooth errors or connection issues

✅ **Always connected:**
- No device scanning or pairing needed
- No reconnection logic triggered
- Instant startup

## Development Workflow

1. **Start mock server:**
   ```bash
   cargo run --release --no-default-features --features mock
   ```

2. **Open browser:**
   ```
   http://localhost:8080
   ```

3. **Edit frontend:**
   - Modify files in `static/` directory
   - Refresh browser to see changes
   - Test with simulated data

4. **Test on Raspberry Pi:**
   - Deploy code to Pi
   - Build with default features (real Bluetooth)
   - Test with actual hardware

## API Examples

```bash
# Get current status (data changes in real-time)
curl http://localhost:8080/status

# Set power to 150W
curl -X POST http://localhost:8080/power/150

# Get current power
curl http://localhost:8080/power

# List training programs
curl http://localhost:8080/programs
```

## Files Added for Mock Support

- `src/mock_bike_controller.rs` - Mock implementation
- `MOCK_MODE.md` - Detailed documentation
- `run-mock.sh` - Helper script
- Updated `Cargo.toml` with feature flags
- Updated `src/main.rs` for conditional compilation

## Switching Modes

| Mode | Command | Use Case |
|------|---------|----------|
| Mock | `cargo run --no-default-features --features mock` | Development on macOS |
| Real | `cargo run` | Production on Raspberry Pi |

## Need Help?

- Mock mode details: `MOCK_MODE.md`
- Project architecture: `CLAUDE.md`
- General info: `README.md`

Happy coding! 🚀
