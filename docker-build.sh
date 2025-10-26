#!/bin/bash

# Script pour compiler avec Docker (évite les problèmes de dépendances)
# Usage: ./docker-build.sh

set -e

echo "🐳 Construction de l'image Docker..."
docker build -t elliptical-cross -f Dockerfile.cross .

echo ""
echo "🔨 Compilation pour ARM64 avec Docker..."
docker run --rm -v "$(pwd)":/app elliptical-cross

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Compilation réussie !"
    echo ""
    echo "Le binaire est disponible dans:"
    echo "  target/aarch64-unknown-linux-gnu/release/elliptical_server"
    echo ""
    echo "Pour déployer sur le Raspberry Pi, utilisez:"
    echo "  ./deploy-to-pi.sh [user@hostname]"
else
    echo ""
    echo "❌ Échec de la compilation"
    exit 1
fi
