#!/bin/bash
# Log cleanup script for Raspberry Pi

echo "🧹 Cleaning logs..."

# Clean journald logs (keep only 3 days)
sudo journalctl --vacuum-time=3d

# Clean logs larger than 50 MB
sudo journalctl --vacuum-size=50M

# Check disk space used by logs
echo ""
echo "📊 Disk space used by logs:"
journalctl --disk-usage

echo ""
echo "✅ Cleanup completed"
