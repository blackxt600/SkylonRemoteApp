# Migration Guide - Fix Autostart Issues

## Problem

The old `startup-command.service` fails at boot because it tries to launch `lxterminal` (GUI terminal) which cannot open display at system startup.

Error: `Gtk-WARNING **: cannot open display: :0`

## Solution

Install the new `skylon-remote.service` which runs directly without GUI terminal.

---

## Migration Steps (Run on Raspberry Pi)

### 1. Pull the latest changes

```bash
cd /home/skylon/Documents/SkylonRemoteApp
git pull
```

### 2. Stop and remove the old service

```bash
# Stop the old service
sudo systemctl stop startup-command.service

# Disable autostart
sudo systemctl disable startup-command.service

# Remove the old service file
sudo rm /etc/systemd/system/startup-command.service

# Reload systemd
sudo systemctl daemon-reload
```

### 3. Install the new service

```bash
# Copy the new service file
sudo cp autostart/skylon-remote.service /etc/systemd/system/

# Reload systemd
sudo systemctl daemon-reload

# Enable autostart
sudo systemctl enable skylon-remote.service

# Start the service now
sudo systemctl start skylon-remote.service
```

### 4. Verify it works

```bash
# Check status (should show "active (running)" in green)
sudo systemctl status skylon-remote.service

# View live logs
sudo journalctl -u skylon-remote.service -f
```

### 5. Test reboot

```bash
# Reboot the Raspberry Pi
sudo reboot
```

After reboot, verify the service started automatically:

```bash
# Check service status
sudo systemctl status skylon-remote.service

# Test the web interface
# Open browser to http://raspberrypi.local:8080
```

### 6. Test Reboot Button in Web Interface

1. Open the web interface: `http://raspberrypi.local:8080`
2. Click the "REBOOT" button
3. Wait for the Raspberry Pi to reboot
4. The service should start automatically after reboot
5. The web interface should be accessible again

---

## What Changed?

### Old service (startup-command.service) ❌
- Tried to launch GUI terminal (`lxterminal`)
- Failed because display not available at boot
- Depended on `graphical.target`
- Hardcoded paths to `/home/skylon`

### New service (skylon-remote.service) ✅
- Runs directly without GUI (`cargo run --release`)
- Works at boot (no display needed)
- Depends on `network.target` and `bluetooth.target`
- Uses portable paths (`%h` variable)
- Logs to journald (viewable with `journalctl`)

---

## Troubleshooting

### Service still fails after migration

Check the logs:
```bash
sudo journalctl -u skylon-remote.service -n 50
```

### Wrong user in service file

If your username is not `skylon`, edit the service file:

```bash
sudo nano /etc/systemd/system/skylon-remote.service
```

Change `User=gilles` to your username, then:

```bash
sudo systemctl daemon-reload
sudo systemctl restart skylon-remote.service
```

### Bluetooth permissions

Add your user to the bluetooth group:

```bash
sudo usermod -a -G bluetooth skylon
```

Log out and log back in for the change to take effect.

---

## Quick Command Summary

Run all commands in sequence:

```bash
cd /home/skylon/Documents/SkylonRemoteApp
git pull
sudo systemctl stop startup-command.service
sudo systemctl disable startup-command.service
sudo rm /etc/systemd/system/startup-command.service
sudo cp autostart/skylon-remote.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable skylon-remote.service
sudo systemctl start skylon-remote.service
sudo systemctl status skylon-remote.service
```

Then test with:
```bash
sudo reboot
```
