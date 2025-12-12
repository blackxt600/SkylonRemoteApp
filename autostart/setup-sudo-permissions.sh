#!/bin/bash
# Automatic configuration of sudo permissions for system shutdown/reboot
# This allows the SkylonRemoteApp to shutdown/reboot the Raspberry Pi without password

set -e  # Exit on error

echo "========================================"
echo "SkylonRemoteApp - Sudo Permissions Setup"
echo "========================================"
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then
   echo "ERROR: Please run this script as a normal user (not with sudo)"
   echo "Usage: ./setup-sudo-permissions.sh"
   echo ""
   echo "The script will ask for your sudo password when needed."
   exit 1
fi

# Get current user
CURRENT_USER="$USER"

echo "Detected user: $CURRENT_USER"
echo ""

# Create temporary sudoers file
TEMP_SUDOERS=$(mktemp)
cat > "$TEMP_SUDOERS" << EOF
# Allow user '$CURRENT_USER' to execute shutdown and reboot without password
# Used by SkylonRemoteApp for system control

# Shutdown command
$CURRENT_USER ALL=(ALL) NOPASSWD: /sbin/shutdown

# Reboot command
$CURRENT_USER ALL=(ALL) NOPASSWD: /sbin/reboot
EOF

echo "Generated sudoers configuration for user: $CURRENT_USER"
echo ""
echo "Content:"
cat "$TEMP_SUDOERS"
echo ""

# Validate sudoers syntax
echo "Validating sudoers syntax..."
if ! sudo visudo -c -f "$TEMP_SUDOERS" > /dev/null 2>&1; then
    echo "ERROR: Invalid sudoers syntax!"
    rm "$TEMP_SUDOERS"
    exit 1
fi
echo "  ✓ Syntax validation passed"
echo ""

# Install sudoers file
echo "Installing sudoers configuration..."
sudo cp "$TEMP_SUDOERS" /etc/sudoers.d/elliptical-server
sudo chmod 0440 /etc/sudoers.d/elliptical-server
sudo chown root:root /etc/sudoers.d/elliptical-server
rm "$TEMP_SUDOERS"
echo "  ✓ File installed to /etc/sudoers.d/elliptical-server"
echo ""

# Test the configuration
echo "Testing configuration..."
if sudo -n /sbin/shutdown --help > /dev/null 2>&1; then
    echo "  ✓ Shutdown permission: OK"
else
    echo "  ✗ Shutdown permission: FAILED"
fi

if sudo -n /sbin/reboot --help > /dev/null 2>&1; then
    echo "  ✓ Reboot permission: OK"
else
    echo "  ✗ Reboot permission: FAILED"
fi
echo ""

echo "========================================"
echo "Setup completed successfully!"
echo "========================================"
echo ""
echo "User '$CURRENT_USER' can now execute:"
echo "  - sudo /sbin/shutdown -h now  (shutdown without password)"
echo "  - sudo /sbin/reboot           (reboot without password)"
echo ""
echo "The SkylonRemoteApp web interface can now use the shutdown/reboot buttons."
echo ""
