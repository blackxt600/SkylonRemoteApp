# Elliptical Server - Serveur de Contrôle Vélo Elliptique Kettler

Serveur HTTP en Rust pour contrôler un vélo elliptique Kettler via Bluetooth avec une interface web moderne.

## 📋 Table des matières

- [Fonctionnalités](#fonctionnalités)
- [Installation](#installation)
- [Utilisation](#utilisation)
- [Programmes d'entraînement](#programmes-dentraînement)
- [API](#api)
- [Développement](#développement)
- [Versioning](#versioning)

## ✨ Fonctionnalités

### Backend
- 🦀 Serveur Rust avec Actix-web
- 🔵 Communication Bluetooth avec vélos elliptiques Kettler (RFCOMM)
- 📡 API REST pour contrôle à distance
- ⚡ Mise à jour en temps réel des données

### Interface Web
- 🎨 Design moderne dark glassmorphisme
- 📱 Responsive pour tablette 11" en mode paysage
- ⏱ Chronomètre avec auto-start/pause basé sur RPM
- 📊 Histogramme visuel de progression
- 🎯 9 programmes d'entraînement prédéfinis
- 🔧 Contrôle de difficulté par paliers de 5W
- 🖥 Mode plein écran
- 📈 Affichage temps réel: RPM, Puissance, État de connexion

## 🚀 Installation

### Prérequis
```bash
# Rust (dernière version stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Bluetooth
sudo apt-get install bluez libbluetooth-dev
```

### Compilation
```bash
# Clone du projet
git clone <votre-repo>
cd elliptical_server

# Build
cargo build --release

# Exécution
cargo run --release
```

Le serveur sera accessible sur `http://0.0.0.0:8080`

## 📱 Utilisation

1. **Connexion Bluetooth** : Associez votre vélo Kettler à `/dev/rfcomm0`
2. **Démarrer le serveur** : `cargo run`
3. **Ouvrir l'interface** : Naviguez vers `http://localhost:8080`
4. **Mode plein écran** : Cliquez sur le bouton ⛶ en haut à droite

### Modes de contrôle

#### Mode Manuel
- Contrôle direct de la puissance avec boutons +/-
- Pas ajustables : 5W, 10W, 25W, 50W
- Plage : 0-250W

#### Mode Programme
- Sélectionnez un des 9 programmes
- Ajustez la difficulté : -100W à +100W par paliers de 5W
- Le chronomètre démarre/pause automatiquement selon votre activité (RPM)

## 🏋️ Programmes d'entraînement

Chaque programme dure **30 minutes** avec ajustement de puissance par minute :

| Programme | Description | Intensité |
|-----------|-------------|-----------|
| **Plat** | Effort constant modéré | ⚡⚡ |
| **Vallée** | Variations douces | ⚡⚡⚡ |
| **Collines** | Deux collines distinctes | ⚡⚡⚡⚡ |
| **Montagne** | Deux sommets | ⚡⚡⚡⚡ |
| **Col Alpin** | Montée progressive | ⚡⚡⚡⚡⚡ |
| **Intervalle** | Intervalles intenses | ⚡⚡⚡⚡⚡ |
| **Pyramide** | Montée et descente symétrique | ⚡⚡⚡⚡ |
| **Changement** | Rythme varié | ⚡⚡⚡ |
| **Altitude** | Variations irrégulières | ⚡⚡⚡⚡ |

## 🔌 API

### GET /status
Récupère l'état actuel du vélo

**Réponse :**
```json
{
  "connected": true,
  "rpm": 65,
  "power": 120,
  "speed": 0.0
}
```

### POST /power/{level}
Définit la puissance cible (0-250W)

**Exemple :**
```bash
curl -X POST http://localhost:8080/power/120
```

## 🛠 Développement

### Structure du projet
```
elliptical_server/
├── src/
│   ├── main.rs              # Serveur HTTP
│   ├── bike_controller.rs   # Contrôleur Bluetooth
│   └── main-example.rs      # Exemple CLI
├── static/
│   └── index.html           # Interface web
├── Cargo.toml               # Dépendances Rust
├── CHANGELOG.md             # Historique des versions
└── VERSION                  # Version actuelle
```

### Dépendances principales
- `actix-web` - Framework web
- `tokio` - Runtime async
- `kdri` - Bibliothèque Kettler Bluetooth
- `serde` - Sérialisation JSON
- `anyhow` - Gestion d'erreurs

## 📦 Versioning

Ce projet utilise [Semantic Versioning](https://semver.org/lang/fr/) (MAJOR.MINOR.PATCH).

### Comment versionner

#### 1. Mettre à jour la version
```bash
# Modifier le fichier VERSION
echo "1.1.0" > VERSION
```

#### 2. Mettre à jour CHANGELOG.md
```markdown
## [1.1.0] - 2025-01-27

### Ajouté
- Nouvelle fonctionnalité X

### Modifié
- Amélioration de Y

### Corrigé
- Bug Z
```

#### 3. Commit et tag
```bash
# Commit des changements
git add -A
git commit -m "Release v1.1.0 - Description des changements"

# Créer le tag
git tag -a v1.1.0 -m "Version 1.1.0"

# Pousser (si dépôt distant)
git push origin master --tags
```

### Convention de versioning

- **MAJOR** (1.x.x) : Changements incompatibles de l'API
- **MINOR** (x.1.x) : Nouvelles fonctionnalités rétrocompatibles
- **PATCH** (x.x.1) : Corrections de bugs rétrocompatibles

### Exemples
```bash
# Bug fix
1.0.0 → 1.0.1

# Nouvelle fonctionnalité
1.0.1 → 1.1.0

# Changement majeur (breaking change)
1.1.0 → 2.0.0
```

### Voir l'historique
```bash
# Liste des versions
git tag -l

# Détails d'une version
git show v1.0.0

# Log avec tags
git log --oneline --decorate

# Différences entre versions
git diff v1.0.0 v1.1.0
```

## 📄 Licence

Ce projet est sous licence MIT.

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request.

## 📞 Support

Pour toute question ou problème, consultez :
- Le fichier [CLAUDE.md](CLAUDE.md) pour les instructions de développement
- Le fichier [CHANGELOG.md](CHANGELOG.md) pour l'historique des versions

---

**Version actuelle :** 1.0.0
**Date :** 2025-01-26
