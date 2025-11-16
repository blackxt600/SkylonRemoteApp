# Installation du script de démarrage automatique sur Raspberry Pi

## Le script est déjà configuré pour :
- Utilisateur : **skylon**
- Commande : `cd /home/skylon/Documents/SkylonRemoteApp && cargo run --release`

## Étape 1 : Copier les fichiers sur la Raspberry Pi

```bash
# Copier le script
scp launch_terminal.sh skylon@raspberry-pi-ip:/home/skylon/

# Rendre le script exécutable
ssh skylon@raspberry-pi-ip "chmod +x /home/skylon/launch_terminal.sh"
```

## Étape 2 : Installer le service systemd

```bash
# Copier le fichier service sur la Raspberry Pi
scp startup-command.service skylon@raspberry-pi-ip:/tmp/

# Se connecter à la Raspberry Pi et installer
ssh skylon@raspberry-pi-ip

# Une fois connecté :
sudo cp /tmp/startup-command.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable startup-command.service

# Démarrer le service maintenant (optionnel, pour tester)
sudo systemctl start startup-command.service
```

## Étape 3 : Configurer la gestion des logs (IMPORTANT pour Raspberry Pi)

Pour éviter que les logs remplissent le disque :

```bash
# Créer le dossier de configuration journald
sudo mkdir -p /etc/systemd/journald.conf.d/

# Copier la configuration de limitation des logs
scp journald-limit.conf skylon@raspberry-pi-ip:/tmp/
ssh skylon@raspberry-pi-ip "sudo cp /tmp/journald-limit.conf /etc/systemd/journald.conf.d/elliptical.conf"

# Redémarrer journald et nettoyer
ssh skylon@raspberry-pi-ip "sudo systemctl restart systemd-journald && sudo journalctl --vacuum-size=50M"
```

**Voir GESTION_LOGS.md pour plus de détails.**

## Étape 4 : Vérifier le statut

```bash
sudo systemctl status startup-command.service

# Vérifier l'utilisation disque des logs
journalctl --disk-usage
```

## Alternative : Autostart (méthode simple sans systemd)

Si vous préférez une méthode plus simple, créez un fichier autostart :

```bash
mkdir -p ~/.config/autostart
nano ~/.config/autostart/elliptical-server.desktop
```

Contenu du fichier .desktop :
```
[Desktop Entry]
Type=Application
Name=Elliptical Server
Exec=/home/skylon/launch_terminal.sh
Terminal=false
```

## Désinstallation

```bash
# Arrêter et désactiver le service
sudo systemctl stop startup-command.service
sudo systemctl disable startup-command.service

# Supprimer le fichier service
sudo rm /etc/systemd/system/startup-command.service
sudo systemctl daemon-reload
```
