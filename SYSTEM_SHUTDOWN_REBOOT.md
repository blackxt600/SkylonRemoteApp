# System Shutdown and Reboot Configuration

This guide explains how to configure sudo permissions to allow the web interface to shutdown and reboot the Raspberry Pi without requiring a password.

## 📋 Table of Contents

- [Overview](#overview)
- [Security Considerations](#security-considerations)
- [Configuration Steps](#configuration-steps)
- [Testing](#testing)
- [Troubleshooting](#troubleshooting)
- [Reverting Changes](#reverting-changes)

## 🔍 Overview

The application provides two system control endpoints:
- **POST /system/shutdown** - Shuts down the Raspberry Pi (`shutdown -h now`)
- **POST /system/reboot** - Reboots the Raspberry Pi (`reboot`)

By default, these commands require root privileges. This guide shows how to configure sudo to allow the user running the server to execute these commands without a password prompt.

## 🔐 Security Considerations

**Important:** Only configure this on a Raspberry Pi that is:
- On a trusted local network
- Not exposed to the internet
- Used for dedicated purposes (like a fitness equipment controller)

### Security Best Practices

1. **Limit to specific user**: Only grant permissions to the user running the application
2. **Limit to specific commands**: Only allow shutdown and reboot, nothing else
3. **No wildcards**: Use exact command paths
4. **Network security**: Ensure your Raspberry Pi is behind a firewall

## ⚙️ Configuration Steps

### Step 1: Identify the User

Determine which user will run the server (default: `skylon`):

```bash
whoami
```

For this guide, we'll use `skylon` as the username. Replace it with your actual username if different.

### Step 2: Create Sudoers Configuration File

Create a dedicated sudoers file (safer than editing the main sudoers file):

```bash
sudo visudo -f /etc/sudoers.d/elliptical-server
```

**Note:** `visudo` validates syntax before saving, preventing configuration errors.

### Step 3: Add Sudo Rules

Add the following content to the file:

```bash
# Allow user 'skylon' to execute shutdown and reboot without password
# Used by SkylonRemoteApp for system control

# Shutdown command
skylon ALL=(ALL) NOPASSWD: /sbin/shutdown

# Reboot command
skylon ALL=(ALL) NOPASSWD: /sbin/reboot
```

**Important:** Replace `skylon` with your actual username if different.

### Step 4: Save and Exit

- In `visudo` (nano editor):
  - Press `Ctrl+X` to exit
  - Press `Y` to confirm save
  - Press `Enter` to confirm filename

### Step 5: Set Correct Permissions

Ensure the file has the correct permissions:

```bash
sudo chmod 0440 /etc/sudoers.d/elliptical-server
```

## ✅ Testing

### Test Shutdown Command (Without Actually Shutting Down)

Test that sudo works without password:

```bash
sudo -n /sbin/shutdown --help
```

If configured correctly:
- ✅ **Success**: Command output appears without password prompt
- ❌ **Failure**: "sudo: a password is required" error

### Test Reboot Command (Without Actually Rebooting)

```bash
sudo -n /sbin/reboot --help
```

### Test Through the Application

1. Start the server:
   ```bash
   cargo run --release
   ```

2. Open the web interface: `http://localhost:8080`

3. Try the shutdown/reboot buttons:
   - Click **🔴 Shutdown** button
   - Confirm the action
   - Check the server logs for success/error messages

**Note:** Actually clicking shutdown will shut down your Raspberry Pi!

## 🔧 Troubleshooting

### Error: "sudo: a password is required"

**Causes:**
1. Sudoers file not created or has syntax errors
2. Wrong username in the configuration
3. File permissions incorrect

**Solution:**
```bash
# Check file exists
ls -la /etc/sudoers.d/elliptical-server

# Check file permissions (should be -r--r-----)
sudo cat /etc/sudoers.d/elliptical-server

# Verify syntax
sudo visudo -c -f /etc/sudoers.d/elliptical-server
```

### Error: "Command not found"

**Cause:** Wrong path to shutdown/reboot commands

**Solution:** Find the correct paths:
```bash
which shutdown
which reboot
```

Update the sudoers file with the correct paths.

### Web Interface Shows Error

**Check server logs:**
```bash
# If running as systemd service
sudo journalctl -u startup-command.service -f

# If running manually
# Check the terminal output where cargo run is running
```

**Common errors:**
- Command execution failed → Check sudo configuration
- Permission denied → Check file permissions
- Timeout → 2-second delay is normal (allows HTTP response before shutdown)

## 🔄 Reverting Changes

To remove the sudo permissions:

```bash
# Remove the sudoers file
sudo rm /etc/sudoers.d/elliptical-server
```

The shutdown and reboot buttons will still appear in the web interface but will fail with permission errors.

## 📝 Alternative: Systemd Service Method

For better security, you can create dedicated systemd services instead of using sudo:

### Create Shutdown Service

```bash
sudo nano /etc/systemd/system/elliptical-shutdown.service
```

Content:
```ini
[Unit]
Description=Elliptical Server Controlled Shutdown

[Service]
Type=oneshot
ExecStart=/sbin/shutdown -h now

[Install]
WantedBy=multi-user.target
```

### Create Reboot Service

```bash
sudo nano /etc/systemd/system/elliptical-reboot.service
```

Content:
```ini
[Unit]
Description=Elliptical Server Controlled Reboot

[Service]
Type=oneshot
ExecStart=/sbin/reboot

[Install]
WantedBy=multi-user.target
```

### Configure Sudo for Services

```bash
sudo visudo -f /etc/sudoers.d/elliptical-server
```

Content:
```bash
skylon ALL=(ALL) NOPASSWD: /bin/systemctl start elliptical-shutdown.service
skylon ALL=(ALL) NOPASSWD: /bin/systemctl start elliptical-reboot.service
```

### Update Server Code

You would need to modify `src/main.rs` to use:
```rust
Command::new("sudo")
    .args(["systemctl", "start", "elliptical-shutdown.service"])
    .spawn()
```

This method provides better audit logging through systemd.

## 📚 Additional Resources

- [Sudo manual](https://www.sudo.ws/man/sudoers.man.html)
- [Systemd documentation](https://www.freedesktop.org/software/systemd/man/)
- [Raspberry Pi security best practices](https://www.raspberrypi.org/documentation/configuration/security.md)

## ⚠️ Final Warning

These commands will **actually shutdown or reboot** your Raspberry Pi when triggered from the web interface. Make sure:
- You've saved any work
- No critical processes are running
- You have physical access to the Raspberry Pi (to power it back on after shutdown)

---

**Configured for user:** `skylon`
**Permissions:** Passwordless sudo for `/sbin/shutdown` and `/sbin/reboot`
**Security level:** Local network only, trusted environment
