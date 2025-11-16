# Log Management on Raspberry Pi

## Problem

The application generates logs that can fill up the Raspberry Pi's disk:
- Bluetooth connection attempts every 30 seconds
- Frequent updates
- Program status messages

On a Raspberry Pi with limited disk space, **journald can use 200-300 MB** without limitation.

## Recommended Solutions

### Solution 1: Limit journald (STRONGLY RECOMMENDED)

```bash
# Copy the configuration
sudo cp journald-limit.conf /etc/systemd/journald.conf.d/elliptical.conf

# Restart journald
sudo systemctl restart systemd-journald

# Clean old logs immediately
sudo journalctl --vacuum-size=50M
```

This limits disk usage to **50 MB maximum**.

### Solution 2: Weekly cleanup script

```bash
# Make the script executable
chmod +x cleanup-logs.sh

# Add to crontab for weekly execution (every Sunday at 2 AM)
sudo crontab -e
```

Add this line:
```
0 2 * * 0 /home/skylon/Documents/SkylonRemoteApp/autostart/cleanup-logs.sh
```

### Solution 3: Completely disable service logs (NOT RECOMMENDED)

If you really don't need logs:

```bash
# Use the service with minimal logs
sudo cp startup-command-minimal-logs.service /etc/systemd/system/startup-command.service

# Modify to use StandardOutput=null and StandardError=null
```

⚠️ **Warning**: You will no longer be able to diagnose Bluetooth connection issues!

## Check disk space usage

```bash
# View log usage
journalctl --disk-usage

# View total disk space
df -h

# View service logs
sudo journalctl -u startup-command.service --since "1 hour ago"
```

## Manual cleanup

```bash
# Clean logs older than 3 days
sudo journalctl --vacuum-time=3d

# Clean to keep only 30 MB
sudo journalctl --vacuum-size=30M

# Delete all archived logs
sudo journalctl --rotate
sudo journalctl --vacuum-time=1s
```

## Final Recommendation

For a Raspberry Pi with limited space:

1. ✅ **Apply journald configuration** (limit to 50 MB)
2. ✅ **Configure weekly cleanup** via cron
3. 💡 **Check disk space regularly** with `df -h`

With these measures, logs will never use more than 50 MB.
