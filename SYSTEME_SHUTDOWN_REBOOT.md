# Configuration Shutdown/Reboot sans mot de passe

Ce document explique comment configurer votre Raspberry Pi pour permettre au serveur elliptique d'arrêter ou de redémarrer le système sans demander de mot de passe.

## ⚠️ Contexte

L'application web propose deux boutons pour gérer le Raspberry Pi :
- **🔴 Éteindre** : Arrête complètement le système (`shutdown -h now`)
- **🔄 Redémarrer** : Redémarre le système (`reboot`)

Par défaut, ces commandes nécessitent des privilèges root (sudo) et demandent un mot de passe. Pour permettre au serveur web d'exécuter ces commandes, il faut configurer sudo.

## 📋 Configuration

### Étape 1 : Identifier l'utilisateur

Le serveur s'exécute sous l'utilisateur qui le lance (généralement `pi` ou votre nom d'utilisateur). Vérifiez avec :

```bash
whoami
```

### Étape 2 : Créer un fichier sudoers

Créez un fichier de configuration sudoers spécifique pour le serveur elliptique :

```bash
sudo visudo -f /etc/sudoers.d/elliptical-server
```

### Étape 3 : Ajouter les permissions

Ajoutez les lignes suivantes (remplacez `pi` par votre nom d'utilisateur) :

```
# Permettre à l'utilisateur pi d'exécuter shutdown et reboot sans mot de passe
pi ALL=(ALL) NOPASSWD: /sbin/shutdown
pi ALL=(ALL) NOPASSWD: /sbin/reboot
```

Enregistrez et quittez l'éditeur (Ctrl+X, puis Y, puis Entrée).

### Étape 4 : Vérifier les permissions du fichier

Le fichier doit avoir les bonnes permissions :

```bash
sudo chmod 0440 /etc/sudoers.d/elliptical-server
```

### Étape 5 : Tester la configuration

Testez que la configuration fonctionne :

```bash
# Test shutdown (n'exécute pas réellement l'arrêt, juste une vérification)
sudo -n shutdown --help

# Test reboot (n'exécute pas réellement le redémarrage, juste une vérification)
sudo -n reboot --help
```

Si ces commandes s'exécutent sans demander de mot de passe, la configuration est correcte.

## ✅ Vérification

1. Démarrez le serveur :
   ```bash
   cargo run --release
   ```

2. Ouvrez l'interface web : `http://localhost:8080`

3. Cliquez sur le bouton **🔄 Redémarrer** (bas gauche de l'écran)

4. Confirmez l'action dans la boîte de dialogue

5. Le Raspberry Pi devrait redémarrer après 2 secondes

## 🔒 Sécurité

**Important** : Cette configuration permet à votre utilisateur d'exécuter `shutdown` et `reboot` sans mot de passe. C'est sécurisé tant que :

- Seul votre utilisateur a accès à l'application
- Le serveur est protégé sur votre réseau local
- Vous ne donnez pas accès à l'interface web depuis Internet sans authentification

### Amélioration de la sécurité (optionnel)

Si vous souhaitez limiter davantage, vous pouvez créer un utilisateur dédié pour le serveur :

```bash
# Créer un utilisateur dédié
sudo useradd -r -s /bin/false elliptical

# Modifier /etc/sudoers.d/elliptical-server
elliptical ALL=(ALL) NOPASSWD: /sbin/shutdown, /sbin/reboot

# Lancer le serveur sous cet utilisateur
sudo -u elliptical cargo run --release
```

## 📱 Utilisation des boutons

### Bouton Éteindre (🔴)
- Position : Bas gauche de l'écran
- Couleur : Rouge
- Action : Arrêt complet du Raspberry Pi
- Délai : 2 secondes après confirmation
- Utilisation : Cliquez, confirmez, attendez que le système s'éteigne

### Bouton Redémarrer (🔄)
- Position : Bas gauche de l'écran (au-dessus du bouton Éteindre)
- Couleur : Orange
- Action : Redémarrage du Raspberry Pi
- Délai : 2 secondes après confirmation
- Utilisation : Cliquez, confirmez, attendez le redémarrage (30-60 secondes)

## 🛠️ Dépannage

### Erreur "sudo: no tty present and no askpass program specified"

Cela signifie que la configuration sudo n'est pas correcte. Vérifiez :
1. Le fichier `/etc/sudoers.d/elliptical-server` existe
2. Le nom d'utilisateur est correct
3. Les chemins complets des commandes sont corrects (`/sbin/shutdown`, `/sbin/reboot`)

### Les boutons ne fonctionnent pas

1. Ouvrez la console développeur du navigateur (F12)
2. Vérifiez s'il y a des erreurs JavaScript
3. Vérifiez que les endpoints `/system/shutdown` et `/system/reboot` répondent
4. Testez manuellement avec curl :
   ```bash
   curl -X POST http://localhost:8080/system/reboot
   ```

### Le système ne redémarre/s'éteint pas

1. Vérifiez les logs du serveur
2. Testez les commandes manuellement :
   ```bash
   sudo shutdown -h now
   sudo reboot
   ```
3. Vérifiez les permissions dans `/etc/sudoers.d/elliptical-server`

## 📚 Références

- Documentation sudo : `man sudoers`
- Documentation shutdown : `man shutdown`
- Documentation reboot : `man reboot`
