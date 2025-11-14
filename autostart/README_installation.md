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

## Étape 3 : Vérifier le statut

```bash
sudo systemctl status startup-command.service
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
