#!/bin/bash
# Automatic installation of skylon-remote.service with correct user configuration

set -e  # Exit on error

echo "========================================"
echo "SkylonRemoteApp Service Installation"
echo "========================================"
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then
   echo "ERROR: Please run this script as a normal user (not with sudo)"
   echo "Usage: ./install-service.sh"
   exit 1
fi

# Get current user
CURRENT_USER="$USER"
CURRENT_HOME="$HOME"

# Get the current directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Detected configuration:"
echo "  User: $CURRENT_USER"
echo "  Home: $CURRENT_HOME"
echo "  Project: $PROJECT_DIR"
echo ""

# Check if cargo exists
if [ ! -f "$CURRENT_HOME/.cargo/bin/cargo" ]; then
    echo "WARNING: Cargo not found at $CURRENT_HOME/.cargo/bin/cargo"
    echo "Please install Rust first: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    read -p "Continue anyway? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Create temporary service file with correct user
TEMP_SERVICE=$(mktemp)
cat > "$TEMP_SERVICE" << EOF
[Unit]
Description=SkylonRemoteApp - Rust HTTP server for Kettler elliptical trainer
After=network.target bluetooth.target
Wants=bluetooth.target

[Service]
Type=simple
User=$CURRENT_USER
WorkingDirectory=%h/Documents/SkylonRemoteApp
ExecStart=%h/.cargo/bin/cargo run --release
Restart=on-failure
RestartSec=10

# Log Management
# By default, logs are sent to journald
StandardOutput=journal
StandardError=journal

# Uncomment to reduce log verbosity (only warnings and errors)
#SyslogLevel=warning

# Uncomment to completely disable logs (NOT RECOMMENDED - makes debugging impossible)
#StandardOutput=null
#StandardError=null

[Install]
WantedBy=multi-user.target
EOF

echo "Generated service file for user: $CURRENT_USER"
echo ""

# Stop old services if they exist
echo "Stopping old services (if any)..."
sudo systemctl stop startup-command.service 2>/dev/null || true
sudo systemctl stop skylon-remote.service 2>/dev/null || true
echo ""

# Disable old services
echo "Disabling old services (if any)..."
sudo systemctl disable startup-command.service 2>/dev/null || true
sudo systemctl disable skylon-remote.service 2>/dev/null || true
echo ""

# Remove old service files
echo "Removing old service files..."
sudo rm -f /etc/systemd/system/startup-command.service
sudo rm -f /etc/systemd/system/skylon-remote.service
echo ""

# Install new service
echo "Installing new service..."
sudo cp "$TEMP_SERVICE" /etc/systemd/system/skylon-remote.service
sudo chmod 644 /etc/systemd/system/skylon-remote.service
rm "$TEMP_SERVICE"
echo "  Service file installed to /etc/systemd/system/skylon-remote.service"
echo ""

# Reload systemd
echo "Reloading systemd..."
sudo systemctl daemon-reload
echo ""

# Enable service
echo "Enabling service for autostart..."
sudo systemctl enable skylon-remote.service
echo ""

# Start service
echo "Starting service..."
sudo systemctl start skylon-remote.service
echo ""

# Wait a moment for service to start
sleep 2

echo "========================================"
echo "Installation completed!"
echo "========================================"
echo ""
echo "Service status:"
echo ""

sudo systemctl status skylon-remote.service --no-pager || true

echo ""
echo "========================================"
echo "Useful commands:"
echo "========================================"
echo ""
echo "Check status:      sudo systemctl status skylon-remote.service"
echo "View logs:         sudo journalctl -u skylon-remote.service -f"
echo "Stop service:      sudo systemctl stop skylon-remote.service"
echo "Start service:     sudo systemctl start skylon-remote.service"
echo "Restart service:   sudo systemctl restart skylon-remote.service"
echo "Disable autostart: sudo systemctl disable skylon-remote.service"
echo ""
echo "Test web interface: http://$(hostname).local:8080"
echo ""
echo "To test autostart after reboot: sudo reboot"
echo ""
