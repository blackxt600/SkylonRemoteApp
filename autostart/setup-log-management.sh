#!/bin/bash
# Automatic configuration of log management for SkylonRemoteApp
# Limits journald logs to prevent disk space issues on Raspberry Pi

set -e  # Exit on error

echo "========================================"
echo "SkylonRemoteApp - Log Management Setup"
echo "========================================"
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then
   echo "ERROR: Please run this script as a normal user (not with sudo)"
   echo "Usage: ./setup-log-management.sh"
   echo ""
   echo "The script will ask for your sudo password when needed."
   exit 1
fi

# Get current user
CURRENT_USER="$USER"
CURRENT_HOME="$HOME"

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "Detected configuration:"
echo "  User: $CURRENT_USER"
echo "  Home: $CURRENT_HOME"
echo "  Script directory: $SCRIPT_DIR"
echo ""

# Check if journald-limit.conf exists
if [ ! -f "$SCRIPT_DIR/journald-limit.conf" ]; then
    echo "ERROR: journald-limit.conf not found in $SCRIPT_DIR"
    exit 1
fi

echo "========================================"
echo "Step 1: Current disk usage"
echo "========================================"
echo ""
echo "Current log disk usage:"
journalctl --disk-usage
echo ""

echo "Available disk space:"
df -h / | grep -v Filesystem
echo ""

read -p "Continue with log management setup? [Y/n] " -n 1 -r
echo
if [[ $REPLY =~ ^[Nn]$ ]]; then
    echo "Setup cancelled."
    exit 0
fi
echo ""

echo "========================================"
echo "Step 2: Install journald limits"
echo "========================================"
echo ""

# Create journald config directory if it doesn't exist
sudo mkdir -p /etc/systemd/journald.conf.d/

# Copy configuration file
echo "Installing journald configuration..."
sudo cp "$SCRIPT_DIR/journald-limit.conf" /etc/systemd/journald.conf.d/skylon.conf
sudo chmod 644 /etc/systemd/journald.conf.d/skylon.conf
echo "  ✓ Configuration installed to /etc/systemd/journald.conf.d/skylon.conf"
echo ""

echo "Configuration content:"
cat "$SCRIPT_DIR/journald-limit.conf"
echo ""

echo "========================================"
echo "Step 3: Restart systemd-journald"
echo "========================================"
echo ""

echo "Restarting journald to apply new limits..."
sudo systemctl restart systemd-journald
echo "  ✓ systemd-journald restarted"
echo ""

# Wait for journald to restart
sleep 2

echo "========================================"
echo "Step 4: Clean existing logs"
echo "========================================"
echo ""

echo "Cleaning logs older than 3 days..."
sudo journalctl --vacuum-time=3d
echo ""

echo "Limiting log size to 50 MB..."
sudo journalctl --vacuum-size=50M
echo ""

echo "Rotating logs..."
sudo journalctl --rotate
echo ""

echo "========================================"
echo "Step 5: Verify configuration"
echo "========================================"
echo ""

echo "New disk usage:"
journalctl --disk-usage
echo ""

echo "========================================"
echo "Step 6: Weekly cleanup (Optional)"
echo "========================================"
echo ""

echo "Would you like to set up automatic weekly log cleanup?"
echo "This will run cleanup-logs.sh every Sunday at 2 AM via cron."
echo ""
read -p "Set up weekly cleanup? [y/N] " -n 1 -r
echo
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Make cleanup script executable
    chmod +x "$SCRIPT_DIR/cleanup-logs.sh"
    echo "  ✓ cleanup-logs.sh made executable"

    # Check if cron job already exists
    CRON_CMD="0 2 * * 0 $SCRIPT_DIR/cleanup-logs.sh >> $CURRENT_HOME/log-cleanup.log 2>&1"

    if crontab -l 2>/dev/null | grep -q "cleanup-logs.sh"; then
        echo "  ℹ Cron job already exists, skipping..."
    else
        # Add to crontab
        (crontab -l 2>/dev/null; echo "$CRON_CMD") | crontab -
        echo "  ✓ Cron job added (runs every Sunday at 2 AM)"
        echo "  ✓ Cleanup logs will be written to: $CURRENT_HOME/log-cleanup.log"
    fi

    echo ""
    echo "To verify cron job:"
    echo "  crontab -l"
    echo ""
    echo "To remove cron job later:"
    echo "  crontab -e"
    echo "  (then delete the line with cleanup-logs.sh)"
    echo ""
else
    echo "  Skipped weekly cleanup setup"
    echo ""
    echo "You can run manual cleanup anytime with:"
    echo "  $SCRIPT_DIR/cleanup-logs.sh"
    echo ""
fi

echo "========================================"
echo "Setup completed successfully!"
echo "========================================"
echo ""
echo "Summary:"
echo "  ✓ Journald limited to 50 MB maximum"
echo "  ✓ Logs rotated daily"
echo "  ✓ Retention period: 1 week"
echo "  ✓ Existing logs cleaned"
echo ""
echo "Useful commands:"
echo ""
echo "View logs (real-time):"
echo "  sudo journalctl -u skylon-remote.service -f"
echo ""
echo "Check disk usage:"
echo "  journalctl --disk-usage"
echo ""
echo "Manual cleanup:"
echo "  $SCRIPT_DIR/cleanup-logs.sh"
echo ""
echo "View cleanup log (if weekly cleanup enabled):"
echo "  cat $CURRENT_HOME/log-cleanup.log"
echo ""
