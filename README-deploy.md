# Déploiement sur Raspberry Pi

## Méthodes de compilation

### Méthode 1 : Compilation sur le Raspberry Pi (⭐ Recommandé)

**Avantages** : Pas de problèmes de dépendances, compilation native simple

```bash
./build-on-pi.sh [user@hostname]
```

Exemples :
```bash
./build-on-pi.sh pi@192.168.1.100
./build-on-pi.sh  # utilise pi@raspberrypi.local par défaut
```

Le script :
1. Copie les sources sur le Pi (rsync)
2. Compile directement sur le Pi (cargo build --release)
3. Le binaire est prêt à être lancé

**Prérequis sur le Raspberry Pi** :
```bash
# Installer Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Installer les dépendances Bluetooth
sudo apt-get update
sudo apt-get install libbluetooth-dev build-essential
```

### Méthode 2 : Cross-compilation avec Docker (⭐ Recommandé pour CI/CD)

**Avantages** : Pas d'installation de dépendances ARM64 sur votre machine, environnement isolé

```bash
./docker-build.sh
./deploy-to-pi.sh [user@hostname]
```

**Prérequis** : Docker installé

### Méthode 3 : Cross-compilation native (Advanced)

**Avantages** : Plus rapide une fois configuré

**Prérequis sur votre machine de développement** :

1. **Installer le cross-compiler ARM64** :
```bash
sudo apt-get update
sudo apt-get install gcc-aarch64-linux-gnu

# ATTENTION: Cette commande peut créer des conflits
# Utilisez plutôt Docker ou compilez sur le Pi
sudo dpkg --add-architecture arm64
sudo apt-get update
sudo apt-get install libbluetooth-dev:arm64
```

2. **Vérifier l'installation** :
```bash
aarch64-linux-gnu-gcc --version
```

### Sur le Raspberry Pi

1. **Configurer le périphérique Bluetooth** :
```bash
sudo rfcomm bind 0 <ADRESSE_MAC_APPAREIL> 1
```

2. **Vérifier le périphérique** :
```bash
ls -l /dev/rfcomm0
```

## Déploiement

### Méthode automatique (recommandée)

```bash
./deploy-to-pi.sh [user@hostname]
```

Exemples :
```bash
./deploy-to-pi.sh pi@192.168.1.100
./deploy-to-pi.sh pi@raspberrypi.local
./deploy-to-pi.sh  # utilise pi@raspberrypi.local par défaut
```

### Méthode manuelle

1. **Compiler pour ARM64** :
```bash
cargo build --release --target=aarch64-unknown-linux-gnu
```

2. **Copier les fichiers** :
```bash
scp target/aarch64-unknown-linux-gnu/release/elliptical_server pi@raspberrypi.local:~/
scp -r static pi@raspberrypi.local:~/
```

3. **Sur le Raspberry Pi** :
```bash
ssh pi@raspberrypi.local
chmod +x elliptical_server
sudo ./elliptical_server
```

## Lancement du serveur

```bash
sudo ./elliptical_server
```

Le serveur sera accessible sur : `http://raspberrypi.local:8080`

## Service systemd (optionnel)

Pour lancer le serveur automatiquement au démarrage :

1. **Créer le fichier service** :
```bash
sudo nano /etc/systemd/system/elliptical.service
```

2. **Contenu du fichier** :
```ini
[Unit]
Description=Elliptical Server
After=network.target bluetooth.target

[Service]
Type=simple
User=root
WorkingDirectory=/home/pi/elliptical_server
ExecStart=/home/pi/elliptical_server/elliptical_server
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

3. **Activer et démarrer le service** :
```bash
sudo systemctl daemon-reload
sudo systemctl enable elliptical.service
sudo systemctl start elliptical.service
sudo systemctl status elliptical.service
```

4. **Voir les logs** :
```bash
sudo journalctl -u elliptical.service -f
```

## Dépannage

### Le binaire ne se lance pas
- Vérifiez l'architecture : `file elliptical_server`
- Devrait afficher : `ELF 64-bit LSB executable, ARM aarch64`

### Erreur de connexion Bluetooth
- Vérifiez `/dev/rfcomm0` existe
- Vérifiez les permissions : `sudo chown root:bluetooth /dev/rfcomm0`

### Le serveur web n'est pas accessible
- Vérifiez que le port 8080 est ouvert : `sudo netstat -tlnp | grep 8080`
- Vérifiez le pare-feu : `sudo ufw status`
