# SkylonRemoteApp - Autostart Configuration Guide

Complete guide for setting up automatic startup and log management on Raspberry Pi.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Service Installation](#service-installation)
3. [Log Management](#log-management)
4. [Troubleshooting](#troubleshooting)
5. [Uninstallation](#uninstallation)

---

## Quick Start

If you just want to get the service running quickly on Raspberry Pi:

```bash
cd $HOME/Documents/SkylonRemoteApp

# Install the service
sudo cp autostart/skylon-remote.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable skylon-remote.service
sudo systemctl start skylon-remote.service

# Configure log limits (RECOMMENDED)
sudo mkdir -p /etc/systemd/journald.conf.d/
sudo cp autostart/journald-limit.conf /etc/systemd/journald.conf.d/skylon.conf
sudo systemctl restart systemd-journald
sudo journalctl --vacuum-size=50M
```

Check status:
```bash
sudo systemctl status skylon-remote.service
```

---

## Service Installation

### System Requirements

- Raspberry Pi OS (or compatible Linux)
- Rust toolchain installed (`cargo` in `$HOME/.cargo/bin/`)
- Bluetooth permissions configured
- Network and Bluetooth enabled

### Configuration Details

The service (`skylon-remote.service`) is configured to:

- **User**: Runs as `gilles` (change in service file if needed)
- **Working Directory**: `$HOME/Documents/SkylonRemoteApp`
- **Command**: `cargo run --release`
- **Dependencies**: Waits for `network.target` and `bluetooth.target`
- **Auto-restart**: Restarts on failure after 10 seconds
- **Logs**: Sent to systemd journald by default

### Step-by-Step Installation

#### 1. Prepare the service file

If your username is not `gilles`, edit the service file first:

```bash
cd $HOME/Documents/SkylonRemoteApp
nano autostart/skylon-remote.service
```

Change `User=gilles` to your username.

#### 2. Install the service

```bash
# Copy service file to systemd directory
sudo cp autostart/skylon-remote.service /etc/systemd/system/

# Reload systemd to recognize the new service
sudo systemctl daemon-reload

# Enable service to start at boot
sudo systemctl enable skylon-remote.service

# Start the service now
sudo systemctl start skylon-remote.service
```

#### 3. Verify installation

```bash
# Check service status (should show "active (running)" in green)
sudo systemctl status skylon-remote.service

# View live logs
sudo journalctl -u skylon-remote.service -f

# Test reboot
sudo reboot
```

After reboot, verify the service started automatically:
```bash
sudo systemctl status skylon-remote.service
```

### Using %h Variable

The service file uses `%h` which systemd replaces with the home directory of the specified user:
- `WorkingDirectory=%h/Documents/SkylonRemoteApp`
- `ExecStart=%h/.cargo/bin/cargo run --release`

This makes the service portable across different users without hardcoded paths.

---

## Log Management

### Why Log Management Matters

The application generates frequent logs:
- Bluetooth connection attempts every 30 seconds
- Device status updates every second
- Training program events
- HTTP requests

**Without log management, journald can consume 200-300 MB** on a Raspberry Pi with limited disk space.

### Solution 1: Limit Journald (STRONGLY RECOMMENDED)

Configure journald to limit disk usage to 50 MB:

```bash
# Create journald configuration directory
sudo mkdir -p /etc/systemd/journald.conf.d/

# Copy the limit configuration
sudo cp $HOME/Documents/SkylonRemoteApp/autostart/journald-limit.conf /etc/systemd/journald.conf.d/skylon.conf

# Restart journald
sudo systemctl restart systemd-journald

# Clean old logs immediately
sudo journalctl --vacuum-size=50M
```

The `journald-limit.conf` file contains:
```ini
[Journal]
SystemMaxUse=50M
SystemMaxFileSize=10M
RuntimeMaxUse=50M
MaxRetentionSec=3day
```

### Solution 2: Weekly Cleanup Script

Set up automatic weekly log cleanup:

```bash
cd $HOME/Documents/SkylonRemoteApp/autostart

# Make the cleanup script executable
chmod +x cleanup-logs.sh

# Add to crontab (runs every Sunday at 2 AM)
crontab -e
```

Add this line to your crontab:
```cron
0 2 * * 0 $HOME/Documents/SkylonRemoteApp/autostart/cleanup-logs.sh
```

Or run manually when needed:
```bash
$HOME/Documents/SkylonRemoteApp/autostart/cleanup-logs.sh
```

### Solution 3: Reduce Log Verbosity

Edit the service file to only log warnings and errors:

```bash
sudo nano /etc/systemd/system/skylon-remote.service
```

Uncomment this line:
```ini
SyslogLevel=warning
```

Then reload:
```bash
sudo systemctl daemon-reload
sudo systemctl restart skylon-remote.service
```

### Solution 4: Disable Logs Completely (NOT RECOMMENDED)

⚠️ **Warning**: This makes debugging Bluetooth issues impossible!

Edit the service file:
```bash
sudo nano /etc/systemd/system/skylon-remote.service
```

Uncomment these lines:
```ini
StandardOutput=null
StandardError=null
```

### Monitoring Disk Space

Check log disk usage:
```bash
# View journald disk usage
journalctl --disk-usage

# View overall disk space
df -h

# View service logs from the last hour
sudo journalctl -u skylon-remote.service --since "1 hour ago"
```

### Manual Log Cleanup

```bash
# Clean logs older than 3 days
sudo journalctl --vacuum-time=3d

# Clean to keep only 30 MB
sudo journalctl --vacuum-size=30M

# Delete all archived logs
sudo journalctl --rotate
sudo journalctl --vacuum-time=1s
```

---

## Troubleshooting

### Service Won't Start

**Check service status:**
```bash
sudo systemctl status skylon-remote.service
```

**View detailed logs:**
```bash
sudo journalctl -u skylon-remote.service -n 100
```

### Common Issues

#### 1. Permission Denied (Bluetooth)

Add your user to the `bluetooth` group:
```bash
sudo usermod -a -G bluetooth $USER
```

Log out and log back in for the change to take effect.

#### 2. Cargo Not Found

Verify cargo is installed:
```bash
$HOME/.cargo/bin/cargo --version
```

If not found, install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 3. Wrong Working Directory

Verify the project path in the service file matches your installation:
```bash
grep WorkingDirectory /etc/systemd/system/skylon-remote.service
```

Should show: `WorkingDirectory=%h/Documents/SkylonRemoteApp`

If the path is different, edit the service file:
```bash
sudo nano /etc/systemd/system/skylon-remote.service
sudo systemctl daemon-reload
sudo systemctl restart skylon-remote.service
```

#### 4. Service Starts Then Stops

This usually indicates an application error. Check logs:
```bash
sudo journalctl -u skylon-remote.service -n 50
```

Test the application manually:
```bash
cd $HOME/Documents/SkylonRemoteApp
$HOME/.cargo/bin/cargo run --release
```

#### 5. Bluetooth Device Not Found

The service will keep retrying connection. Ensure:
- Bluetooth is enabled: `bluetoothctl power on`
- Kettler device is paired and trusted
- Device is powered on and in range

### Reboot Issues (Historical)

**Previous Problem**: The service didn't start after reboot because it tried to launch a GUI terminal (`lxterminal`) that wasn't available during system startup.

**Solution**: The current service (`skylon-remote.service`) runs directly without GUI terminal. Logs are managed by journald instead.

If you have the old service configuration:
1. Stop and disable old service
2. Install the new `skylon-remote.service`
3. The service now depends on `network.target` and `bluetooth.target` instead of `graphical.target`

---

## Uninstallation

To remove the autostart service:

```bash
# Stop the service
sudo systemctl stop skylon-remote.service

# Disable autostart
sudo systemctl disable skylon-remote.service

# Remove the service file
sudo rm /etc/systemd/system/skylon-remote.service

# Reload systemd
sudo systemctl daemon-reload
```

To also remove log management configuration:

```bash
# Remove journald limit configuration
sudo rm /etc/systemd/journald.conf.d/skylon.conf
sudo systemctl restart systemd-journald

# Remove crontab entry (if configured)
crontab -e
# Delete the line with cleanup-logs.sh
```

---

## Optional: GUI Terminal Launch

If you want to see the server output in a graphical terminal window (useful for development), use `launch_terminal.sh`:

```bash
cd $HOME/Documents/SkylonRemoteApp
./autostart/launch_terminal.sh
```

**Note**: This script is NOT used by the systemd service. The service runs without GUI and logs to journald.

---

## Files in This Directory

- **skylon-remote.service** - Main systemd service file
- **journald-limit.conf** - Journald configuration to limit log size
- **cleanup-logs.sh** - Manual/cron log cleanup script
- **launch_terminal.sh** - Optional script to launch in GUI terminal
- **AUTOSTART_SETUP.md** - This documentation file

### Obsolete Files (Replaced)

- ~~startup-command.service~~ - Replaced by `skylon-remote.service`
- ~~startup-command-minimal-logs.service~~ - Merged into `skylon-remote.service`
- ~~README_installation.md~~ - Merged into this file
- ~~GESTION_LOGS.md~~ - Merged into this file
- ~~FIX_REBOOT_ISSUE.md~~ - Merged into this file

---

## Best Practices for Raspberry Pi

For optimal operation on Raspberry Pi with limited disk space:

1. ✅ **Install the service** (`skylon-remote.service`)
2. ✅ **Configure journald limits** (50 MB max)
3. ✅ **Set up weekly log cleanup** (cron job)
4. ✅ **Add user to bluetooth group**
5. ✅ **Test after reboot**
6. 💡 **Monitor disk space** regularly with `df -h`

With these measures:
- Service starts automatically on boot
- Logs never exceed 50 MB
- Bluetooth reconnects automatically
- System remains stable long-term

---

## Support

For issues or questions:
- Check the main README.md in the project root
- Review application logs: `sudo journalctl -u skylon-remote.service -f`
- Test manual execution: `cd $HOME/Documents/SkylonRemoteApp && cargo run --release`
