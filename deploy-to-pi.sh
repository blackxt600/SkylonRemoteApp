#!/bin/bash

# Script de déploiement pour Raspberry Pi
# Usage: ./deploy-to-pi.sh [user@hostname]

set -e

# Configuration
PI_HOST="${1:-pi@raspberrypi.local}"
REMOTE_DIR="~/elliptical_server"
TARGET="aarch64-unknown-linux-gnu"

echo "🔨 Compilation pour Raspberry Pi (ARM64)..."
cargo build --release --target=$TARGET

if [ $? -ne 0 ]; then
    echo "❌ Échec de la compilation"
    exit 1
fi

echo "✅ Compilation réussie"
echo ""
echo "📦 Préparation du déploiement vers $PI_HOST..."

# Créer le répertoire distant si nécessaire
ssh $PI_HOST "mkdir -p $REMOTE_DIR/static"

# Copier le binaire
echo "📤 Copie du binaire..."
scp target/$TARGET/release/elliptical_server $PI_HOST:$REMOTE_DIR/

# Copier les fichiers statiques
echo "📤 Copie des fichiers statiques..."
scp -r static/* $PI_HOST:$REMOTE_DIR/static/

# Rendre le binaire exécutable
ssh $PI_HOST "chmod +x $REMOTE_DIR/elliptical_server"

echo ""
echo "✅ Déploiement terminé !"
echo ""
echo "Pour lancer le serveur sur le Raspberry Pi :"
echo "  ssh $PI_HOST"
echo "  cd $REMOTE_DIR"
echo "  sudo ./elliptical_server"
echo ""
echo "Note: sudo est nécessaire pour accéder au Bluetooth"
