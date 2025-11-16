# Gestion des logs sur Raspberry Pi

## Problème

Le programme génère des logs qui peuvent remplir le disque de la Raspberry Pi :
- Tentatives de connexion Bluetooth toutes les 30 secondes
- Mises à jour fréquentes
- Messages d'état du programme

Sur une Raspberry Pi avec peu d'espace disque, **journald peut utiliser 200-300 Mo** sans limitation.

## Solutions recommandées

### Solution 1 : Limiter journald (FORTEMENT RECOMMANDÉ)

```bash
# Copier la configuration
sudo cp journald-limit.conf /etc/systemd/journald.conf.d/elliptical.conf

# Redémarrer journald
sudo systemctl restart systemd-journald

# Nettoyer les anciens logs immédiatement
sudo journalctl --vacuum-size=50M
```

Cela limite l'utilisation disque à **50 Mo maximum**.

### Solution 2 : Script de nettoyage hebdomadaire

```bash
# Rendre le script exécutable
chmod +x cleanup-logs.sh

# Ajouter à crontab pour exécution hebdomadaire (chaque dimanche à 2h)
sudo crontab -e
```

Ajoutez cette ligne :
```
0 2 * * 0 /home/skylon/Documents/SkylonRemoteApp/autostart/cleanup-logs.sh
```

### Solution 3 : Désactiver complètement les logs du service (NON RECOMMANDÉ)

Si vous n'avez vraiment pas besoin des logs :

```bash
# Utiliser le service avec logs minimaux
sudo cp startup-command-minimal-logs.service /etc/systemd/system/startup-command.service

# Modifier pour utiliser StandardOutput=null et StandardError=null
```

⚠️ **Attention** : Vous ne pourrez plus diagnostiquer les problèmes de connexion Bluetooth !

## Vérifier l'espace disque utilisé

```bash
# Voir l'utilisation des logs
journalctl --disk-usage

# Voir l'espace disque total
df -h

# Voir les logs du service
sudo journalctl -u startup-command.service --since "1 hour ago"
```

## Nettoyage manuel

```bash
# Nettoyer les logs de plus de 3 jours
sudo journalctl --vacuum-time=3d

# Nettoyer pour garder seulement 30 Mo
sudo journalctl --vacuum-size=30M

# Supprimer tous les logs archivés
sudo journalctl --rotate
sudo journalctl --vacuum-time=1s
```

## Recommandation finale

Pour une Raspberry Pi avec peu d'espace :

1. ✅ **Appliquer la configuration journald** (limite à 50 Mo)
2. ✅ **Configurer le nettoyage hebdomadaire** via cron
3. 💡 **Vérifier l'espace disque régulièrement** avec `df -h`

Avec ces mesures, les logs n'utiliseront jamais plus de 50 Mo.
