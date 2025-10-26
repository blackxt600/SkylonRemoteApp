# Changelog

Toutes les modifications notables de ce projet seront documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/),
et ce projet adhère au [Semantic Versioning](https://semver.org/lang/fr/).

## [1.0.0] - 2025-01-26

### Ajouté
- Serveur HTTP Rust avec Actix-web pour contrôler un vélo elliptique Kettler via Bluetooth
- Interface web moderne avec design dark glassmorphisme
- 9 programmes d'entraînement prédéfinis (30 minutes chacun):
  - Plat - Effort constant modéré
  - Vallée - Variations douces
  - Collines - Deux collines
  - Montagne - Deux sommets
  - Col Alpin - Montée progressive
  - Intervalle - Intervalles intenses
  - Pyramide - Montée et descente symétrique
  - Changement - Rythme varié
  - Altitude - Variations irrégulières
- Chronomètre avec auto-start/pause basé sur le RPM
- Contrôle de difficulté par paliers de 5W (-100W à +100W)
- Histogramme visuel avec barres colorées (vert=complété, orange=actuel, violet=futur)
- Mode manuel avec contrôle de puissance par paliers (5W, 10W, 25W, 50W)
- Affichage en temps réel: RPM, Puissance, État de connexion
- Indicateur de connexion visuel (point vert/rouge lumineux)
- Bouton plein écran pour utilisation sur tablette
- Layout responsive optimisé pour tablette 11" en mode paysage
- Auto-ajustement de la puissance selon le programme actif

### Technique
- Backend Rust avec bibliothèque kdri pour protocole Kettler Bluetooth
- Communication asynchrone avec tokio
- API REST avec endpoints /status et /power/{level}
- Interface HTML/CSS/JavaScript moderne
- Support Bluetooth RFCOMM (/dev/rfcomm0)

### Notes
- Version initiale stable
- Testé avec vélo elliptique Kettler
