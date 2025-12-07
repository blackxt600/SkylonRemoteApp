#!/bin/bash
# Automatic migration from old startup-command.service to new skylon-remote.service

set -e  # Exit on error

echo "========================================"
echo "SkylonRemoteApp Service Migration"
echo "========================================"
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then
   echo "ERROR: Please run this script as a normal user (not with sudo)"
   echo "Usage: ./migrate-service.sh"
   exit 1
fi

# Get the current directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Project directory: $PROJECT_DIR"
echo ""

# Check if the new service file exists
if [ ! -f "$SCRIPT_DIR/skylon-remote.service" ]; then
    echo "ERROR: skylon-remote.service not found in $SCRIPT_DIR"
    echo "Please pull the latest changes first: git pull"
    exit 1
fi

echo "Step 1/6: Stopping old service (if exists)..."
sudo systemctl stop startup-command.service 2>/dev/null || echo "  (old service not running or doesn't exist)"

echo ""
echo "Step 2/6: Disabling old service (if exists)..."
sudo systemctl disable startup-command.service 2>/dev/null || echo "  (old service not enabled or doesn't exist)"

echo ""
echo "Step 3/6: Removing old service file..."
if [ -f /etc/systemd/system/startup-command.service ]; then
    sudo rm /etc/systemd/system/startup-command.service
    echo "  Old service file removed"
else
    echo "  (old service file doesn't exist)"
fi

echo ""
echo "Step 4/6: Installing new service..."
sudo cp "$SCRIPT_DIR/skylon-remote.service" /etc/systemd/system/
echo "  New service file copied"

echo ""
echo "Step 5/6: Reloading systemd..."
sudo systemctl daemon-reload
echo "  Systemd reloaded"

echo ""
echo "Step 6/6: Enabling and starting new service..."
sudo systemctl enable skylon-remote.service
sudo systemctl start skylon-remote.service
echo "  Service enabled and started"

echo ""
echo "========================================"
echo "Migration completed successfully!"
echo "========================================"
echo ""
echo "Checking service status..."
echo ""

sudo systemctl status skylon-remote.service --no-pager

echo ""
echo "========================================"
echo "Next steps:"
echo "========================================"
echo ""
echo "1. Check the logs: sudo journalctl -u skylon-remote.service -f"
echo "2. Test the web interface: http://$(hostname).local:8080"
echo "3. Test reboot: sudo reboot"
echo ""
echo "After reboot, verify the service started:"
echo "  sudo systemctl status skylon-remote.service"
echo ""
