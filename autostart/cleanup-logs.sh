#!/bin/bash
# Script de nettoyage des logs pour Raspberry Pi

echo "🧹 Nettoyage des logs..."

# Nettoyer les logs journald (garder seulement 3 jours)
sudo journalctl --vacuum-time=3d

# Nettoyer les logs de plus de 50 Mo
sudo journalctl --vacuum-size=50M

# Vérifier l'espace disque utilisé par les logs
echo ""
echo "📊 Espace disque utilisé par les logs :"
journalctl --disk-usage

echo ""
echo "✅ Nettoyage terminé"
