#!/bin/bash

# Script pour compiler directement sur le Raspberry Pi
# Usage: ./build-on-pi.sh [user@hostname]

set -e

PI_HOST="${1:-pi@raspberrypi.local}"
REMOTE_DIR="~/elliptical_server"

echo "📦 Copie des sources vers $PI_HOST..."

# Copier tout le projet (sauf target et .git)
rsync -av --exclude 'target' --exclude '.git' \
  --exclude '.cargo' \
  . $PI_HOST:$REMOTE_DIR/

echo "✅ Sources copiées"
echo ""
echo "🔨 Compilation sur le Raspberry Pi..."

# Compiler sur le Pi
ssh $PI_HOST "cd $REMOTE_DIR && cargo build --release"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Compilation réussie !"
    echo ""
    echo "Pour lancer le serveur sur le Raspberry Pi :"
    echo "  ssh $PI_HOST"
    echo "  cd $REMOTE_DIR"
    echo "  sudo ./target/release/elliptical_server"
    echo ""
    echo "Note: sudo est nécessaire pour accéder au Bluetooth"
else
    echo ""
    echo "❌ Échec de la compilation"
    exit 1
fi
