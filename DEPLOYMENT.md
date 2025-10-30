# Guide de déploiement sur Raspberry Pi 3 (64-bit)

Ce guide explique comment déployer le serveur Kettler sur une Raspberry Pi 3 avec un système 64-bit.

## Prérequis

- Raspberry Pi 3 avec système d'exploitation 64-bit (Raspberry Pi OS 64-bit ou Ubuntu)
- Connexion Internet
- Appareil Kettler compatible Bluetooth

## Méthode Recommandée : Compilation directe sur la Raspberry Pi

### 1. Installation de Rust et des dépendances

Connectez-vous en SSH à votre Raspberry Pi et exécutez :

```bash
# Installer Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Installer les dépendances Bluetooth et build tools
sudo apt-get update
sudo apt-get install -y bluez libbluetooth-dev build-essential git
```

### 2. Cloner et compiler le projet

```bash
# Cloner le projet depuis GitHub
git clone https://github.com/blackxt600/YpApp.git
cd YpApp

# Compiler en mode release (durée : 15-30 minutes)
cargo build --release

# Le binaire compilé se trouve dans : target/release/elliptical_server
```

### 3. Configuration Bluetooth

#### Scanner les appareils Kettler

```bash
# Activer le Bluetooth
sudo systemctl start bluetooth
sudo systemctl enable bluetooth

# Scanner les appareils disponibles
sudo hcitool scan
```

Notez l'adresse MAC de votre appareil Kettler (format : XX:XX:XX:XX:XX:XX).

#### Lier l'appareil Kettler

```bash
# Lier l'appareil (remplacer XX:XX:XX:XX:XX:XX par votre adresse MAC)
sudo rfcomm bind /dev/rfcomm0 XX:XX:XX:XX:XX:XX 1

# Vérifier la liaison
ls -l /dev/rfcomm0

# Pour rendre la liaison permanente au démarrage
echo "rfcomm0 {
    bind yes;
    device XX:XX:XX:XX:XX:XX;
    channel 1;
    comment \"Kettler Device\";
}" | sudo tee -a /etc/bluetooth/rfcomm.conf
```

### 4. Test du serveur

```bash
# Lancer le serveur (nécessite sudo pour accéder au Bluetooth)
sudo ./target/release/elliptical_server

# Dans un autre terminal SSH, tester l'API
curl http://localhost:8080/status
curl -X POST http://localhost:8080/resistance/10
```

Si tout fonctionne, vous devriez voir les données de votre appareil en réponse.

### 5. Configurer le démarrage automatique avec systemd

#### Créer le fichier de service

```bash
sudo nano /etc/systemd/system/elliptical-server.service
```

Copier le contenu suivant :

```ini
[Unit]
Description=Kettler Elliptical Server
After=network.target bluetooth.target

[Service]
Type=simple
User=root
WorkingDirectory=/home/pi/YpApp
ExecStart=/home/pi/YpApp/target/release/elliptical_server
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

**Note :** Ajustez le chemin `/home/pi/YpApp` si votre projet est dans un autre répertoire.

#### Activer et démarrer le service

```bash
# Recharger systemd pour prendre en compte le nouveau service
sudo systemctl daemon-reload

# Activer le démarrage automatique au boot
sudo systemctl enable elliptical-server

# Démarrer le service immédiatement
sudo systemctl start elliptical-server

# Vérifier le statut du service
sudo systemctl status elliptical-server

# Voir les logs en temps réel
sudo journalctl -u elliptical-server -f
```

#### Commandes utiles pour gérer le service

```bash
# Arrêter le service
sudo systemctl stop elliptical-server

# Redémarrer le service
sudo systemctl restart elliptical-server

# Désactiver le démarrage automatique
sudo systemctl disable elliptical-server

# Voir les logs des dernières 24h
sudo journalctl -u elliptical-server --since "24 hours ago"
```

### 6. Accéder au serveur depuis d'autres appareils

Le serveur écoute sur `0.0.0.0:8080`, ce qui signifie qu'il est accessible depuis n'importe quel appareil sur le réseau local.

Trouver l'adresse IP de la Raspberry Pi :

```bash
hostname -I
```

Depuis un autre appareil sur le même réseau :

```bash
# Remplacer 192.168.1.100 par l'IP de votre Raspberry Pi
curl http://192.168.1.100:8080/status
curl -X POST http://192.168.1.100:8080/resistance/5
```

### 7. Mise à jour du serveur

Pour mettre à jour le serveur avec une nouvelle version :

```bash
cd ~/YpApp

# Récupérer les dernières modifications
git pull origin main

# Recompiler
cargo build --release

# Redémarrer le service
sudo systemctl restart elliptical-server
```

## Méthode Alternative : Cross-compilation depuis votre PC

Si vous préférez compiler sur votre PC Linux pour ensuite transférer le binaire :

### Sur votre PC de développement

```bash
# Installer la cible ARM64
rustup target add aarch64-unknown-linux-gnu

# Installer le cross-compiler
sudo apt-get install -y gcc-aarch64-linux-gnu

# Installer les bibliothèques Bluetooth pour ARM64
sudo dpkg --add-architecture arm64
sudo apt-get update
sudo apt-get install -y libbluetooth-dev:arm64

# Compiler pour Raspberry Pi
cargo build --release --target aarch64-unknown-linux-gnu

# Transférer le binaire vers la Raspberry Pi
scp target/aarch64-unknown-linux-gnu/release/elliptical_server pi@raspberrypi.local:~/
```

### Sur la Raspberry Pi

```bash
# Rendre le binaire exécutable
chmod +x ~/elliptical_server

# Installer uniquement les dépendances runtime
sudo apt-get install -y bluez libbluetooth3

# Tester
sudo ./elliptical_server
```

Ensuite, suivez les étapes 3 à 6 ci-dessus pour la configuration Bluetooth et systemd.

## Dépannage

### Le serveur ne trouve pas l'appareil Bluetooth

```bash
# Vérifier que le Bluetooth est actif
sudo systemctl status bluetooth

# Vérifier que /dev/rfcomm0 existe et est accessible
ls -l /dev/rfcomm0

# Relancer la liaison RFCOMM
sudo rfcomm release /dev/rfcomm0
sudo rfcomm bind /dev/rfcomm0 XX:XX:XX:XX:XX:XX 1
```

### Erreur "Permission denied" sur /dev/rfcomm0

Le serveur doit être exécuté avec sudo ou en tant que root pour accéder au port Bluetooth.

### Le service ne démarre pas

```bash
# Voir les logs d'erreur
sudo journalctl -u elliptical-server -n 50

# Vérifier que le chemin du binaire est correct dans le fichier service
sudo systemctl cat elliptical-server
```

### Port 8080 déjà utilisé

```bash
# Vérifier quel processus utilise le port
sudo lsof -i :8080

# Arrêter le processus si nécessaire
sudo kill <PID>
```

## API du serveur

Une fois le serveur démarré, l'API REST est disponible :

### GET /status

Récupère l'état actuel de l'appareil (vitesse, pulsations, résistance).

```bash
curl http://localhost:8080/status
```

Réponse :
```json
{
  "speed": 12.5,
  "pulse": 140,
  "resistance": 8
}
```

### POST /resistance/{level}

Change le niveau de résistance (0-16 pour les vélos/crosstrainers).

```bash
curl -X POST http://localhost:8080/resistance/10
```

Réponse :
```json
{
  "success": true,
  "new_resistance": 10
}
```

## Support

Pour plus d'informations sur le projet, consultez :
- [README.md](README.md) - Vue d'ensemble du projet
- [CLAUDE.md](CLAUDE.md) - Architecture technique détaillée
- [GitHub Issues](https://github.com/blackxt600/YpApp/issues) - Pour signaler des problèmes
