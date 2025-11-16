# Elliptical Server - Kettler Elliptical Bike Control Server

HTTP server in Rust to control a Kettler elliptical bike via Bluetooth with a modern web interface.

## 📋 Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Training Programs](#training-programs)
- [API](#api)
- [Development](#development)
- [Versioning](#versioning)

## ✨ Features

### Backend
- 🦀 Rust server with Actix-web
- 🔵 Bluetooth communication with Kettler elliptical bikes (RFCOMM)
- 📡 REST API for remote control
- ⚡ Real-time data updates

### Web Interface
- 🎨 Modern dark glassmorphism design
- 📱 Responsive for 11" tablet in landscape mode
- ⏱ Timer with auto-start/pause based on RPM
- 📊 Visual progress histogram
- 🎯 9 predefined training programs
- 🔧 Difficulty control in 5W increments
- 🖥 Fullscreen mode
- 📈 Real-time display: RPM, Power, Connection Status

## 🚀 Installation

### Prerequisites
```bash
# Rust (latest stable version)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Bluetooth
sudo apt-get install bluez libbluetooth-dev
```

### Compilation
```bash
# Clone the project
git clone https://github.com/blackxt600/SkylonRemoteApp.git
cd SkylonRemoteApp

# Build
cargo build --release

# Run
cargo run --release
```

The server will be accessible at `http://0.0.0.0:8080`

## 📱 Usage

1. **Bluetooth Connection**: Pair your Kettler bike to `/dev/rfcomm0`
2. **Start the server**: `cargo run`
3. **Open the interface**: Navigate to `http://localhost:8080`
4. **Fullscreen mode**: Click the ⛶ button in the top right

### Control Modes

#### Manual Mode
- Direct power control with +/- buttons
- Adjustable steps: 5W, 10W, 25W, 50W
- Range: 25-400W

#### Program Mode
- Select one of the 9 programs
- Adjust difficulty: -100W to +100W in 5W increments
- Timer automatically starts/pauses based on your activity (RPM)

## 🏋️ Training Programs

Each program lasts **30 minutes** with power adjustment per minute:

| Program | Description | Intensity |
|---------|-------------|-----------|
| **Flat** | Moderate constant effort | ⚡⚡ |
| **Valley** | Gentle variations | ⚡⚡⚡ |
| **Hills** | Two distinct hills | ⚡⚡⚡⚡ |
| **Mountain** | Two peaks | ⚡⚡⚡⚡ |
| **Alpine Pass** | Progressive climb | ⚡⚡⚡⚡⚡ |
| **Interval** | Intense intervals | ⚡⚡⚡⚡⚡ |
| **Pyramid** | Symmetrical climb and descent | ⚡⚡⚡⚡ |
| **Change** | Varied rhythm | ⚡⚡⚡ |
| **Altitude** | Irregular variations | ⚡⚡⚡⚡ |

## 🔌 API

### GET /status
Get the current bike status

**Response:**
```json
{
  "connected": true,
  "rpm": 65,
  "power": 120,
  "speed": 0.0
}
```

### POST /power/{level}
Set the target power (25-400W)

**Example:**
```bash
curl -X POST http://localhost:8080/power/120
```

## 🛠 Development

### Project Structure
```
SkylonRemoteApp/
├── src/
│   ├── main.rs              # HTTP server
│   ├── bike_controller.rs   # Bluetooth controller
│   ├── training_program.rs  # Training program structures
│   └── main-example.rs      # CLI example
├── static/
│   ├── index.html           # Web interface
│   └── programs.html        # Program manager
├── autostart/               # Systemd autostart config
├── Cargo.toml               # Rust dependencies
├── CHANGELOG.md             # Version history
└── VERSION                  # Current version
```

### Main Dependencies
- `actix-web` - Web framework
- `tokio` - Async runtime
- `kdri` - Kettler Bluetooth library
- `serde` - JSON serialization
- `anyhow` - Error handling

## 📦 Versioning

This project uses [Semantic Versioning](https://semver.org/) (MAJOR.MINOR.PATCH).

### How to Version

#### 1. Update the version
```bash
# Modify the VERSION file
echo "1.1.0" > VERSION
```

#### 2. Update CHANGELOG.md
```markdown
## [1.1.0] - 2025-01-27

### Added
- New feature X

### Changed
- Improvement to Y

### Fixed
- Bug Z
```

#### 3. Commit and tag
```bash
# Commit changes
git add -A
git commit -m "Release v1.1.0 - Description of changes"

# Create tag
git tag -a v1.1.0 -m "Version 1.1.0"

# Push (if remote repository)
git push origin main --tags
```

### Versioning Convention

- **MAJOR** (1.x.x): Incompatible API changes
- **MINOR** (x.1.x): New backward-compatible features
- **PATCH** (x.x.1): Backward-compatible bug fixes

### Examples
```bash
# Bug fix
1.0.0 → 1.0.1

# New feature
1.0.1 → 1.1.0

# Breaking change
1.1.0 → 2.0.0
```

### View History
```bash
# List versions
git tag -l

# Version details
git show v1.0.0

# Log with tags
git log --oneline --decorate

# Differences between versions
git diff v1.0.0 v1.1.0
```

## 🙏 Credits

This project would not have been possible without the incredible work shared in the [kdri](https://github.com/kaegi/kdri) repository by [@kaegi](https://github.com/kaegi). The kdri library (Kettler Device Rust Interface) provides the essential Bluetooth communication protocol implementation for Kettler fitness devices.

Thank you for making this library open source! 🎉

## 📄 License

This project is licensed under the MIT License.

## 🤝 Contribution & Open Source Philosophy

**This repository is shared "as-is" for the community to take, improve, and make their own!**

This project was built to solve a specific need and is now released to the world. **The original author will not provide bug fixes or ongoing maintenance.** Instead, this code is offered as a foundation for you to build upon, learn from, and adapt to your needs.

Open source is about empowerment—taking something that works and making it better, fixing what's broken, and sharing improvements with others who face similar challenges. This project exists because of the incredible work shared by others (like the [kdri](https://github.com/kaegi/kdri) library), and now it's **your turn** to carry the torch forward.

### This Is YOUR Project Now

- 🔧 **Fix bugs**: Found an issue? Fix it and share your solution with others
- 🚀 **Add features**: Need something new? Build it and submit a pull request
- 📚 **Improve docs**: Make the documentation better, clearer, more complete
- 🌍 **Port to new devices**: Make it work with other Kettler models or fitness equipment
- 🎨 **Redesign the UI**: Create a better interface that fits your needs
- 🔬 **Experiment freely**: Try new approaches, test wild ideas, break things and learn

### How to Take Ownership

- **Fork this repository** - Make it yours, customize it, improve it
- **Share your improvements** - Open pull requests so others can benefit
- **Create your own version** - Build something completely different based on this code
- **Help each other** - Answer questions, review PRs, collaborate with other contributors
- **Become a maintainer** - If you're passionate about this project, consider maintaining your own fork

### No Promises, Just Possibilities

⚠️ **Important**: There is no guarantee of support, bug fixes, or updates from the original author. This is community-driven from now on. If you need something fixed or improved, **you** have the power to make it happen.

**This isn't just an invitation to contribute—it's a transfer of ownership to the community.**

The code is here. The tools are available. The possibilities are endless. **What will YOU build with it?** 🚀

## 📞 Support

For questions or issues, see:
- [CLAUDE.md](CLAUDE.md) for development instructions
- [CHANGELOG.md](CHANGELOG.md) for version history
- [autostart/GESTION_LOGS.md](autostart/GESTION_LOGS.md) for log management on Raspberry Pi
- [SYSTEM_SHUTDOWN_REBOOT.md](SYSTEM_SHUTDOWN_REBOOT.md) for shutdown/reboot configuration

---

**Current Version:** 1.8.2
**Date:** 2025-11-16
