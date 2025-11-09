# Changelog

Toutes les modifications notables de ce projet seront documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/),
et ce projet adhère au [Semantic Versioning](https://semver.org/lang/fr/).

## [1.4.0] - 2025-11-09

### Ajouté
- **Éditeur de programmes prédéfinis**
  - Nouveau bouton "⚙️ Éditer" dans l'interface principale
  - Modal d'édition avec visualisation de tous les programmes (Plat, Vallée, Collines, etc.)
  - Prévisualisation graphique sous forme de mini histogrammes
  - Édition des 30 valeurs de puissance par minute (une par minute)
  - Sauvegarde automatique dans localStorage du navigateur
  - Persistance des modifications entre les sessions
  - Bouton de réinitialisation par programme
  - Bouton de réinitialisation globale (tous les programmes)
  - Rechargement automatique si le programme actif est modifié

### Modifié
- **Limite minimale de puissance fixée à 25W** (au lieu de 0W)
  - Validation frontend : curseurs et formulaires (25-400W)
  - Validation backend : API REST `set_power()` (25-400W)
  - Validation création de programmes personnalisés
  - Messages d'erreur mis à jour partout
  - Protection complète sur toutes les fonctionnalités
- **Repositionnement des boutons système**
  - Bouton "🔴 Éteindre" déplacé en bas à gauche
  - Bouton "🔄 Redémarrer" déplacé en bas à droite
  - Symétrie parfaite (même hauteur : 40px du bas)
- **Interface programmes personnalisés** (static/programs.html)
  - Curseur de puissance : min="25" au lieu de min="0"

### Technique
- `static/index.html` : +187 lignes
  - Système localStorage pour sauvegarder les programmes personnalisés
  - Fonctions `loadPrograms()`, `savePrograms()`
  - Fonctions d'édition : `openProgramsEditor()`, `editProgramValues()`, `resetSingleProgram()`, `resetAllPrograms()`
  - Séparation `defaultPrograms` (valeurs d'origine) et `programs` (valeurs actuelles)
- `src/bike_controller.rs` : Validation `set_power()` 25-400W
- `src/training_program.rs` : Validation `is_valid()` avec `power_target >= 25`

## [1.3.1] - 2025-11-09

### Modifié
- **Interface de création de programmes** (static/programs.html)
  - Durée par défaut des intervalles réduite de 5 minutes (300s) à 1 minute (60s)
  - Facilite la création de programmes plus courts et personnalisés
  - Les utilisateurs peuvent toujours ajuster la durée de 30 secondes à 20 minutes

## [1.3.0] - 2025-11-09

### Ajouté
- **Boutons de gestion système** dans l'interface web
  - Bouton **Éteindre** (🔴 rouge) pour arrêt complet du Raspberry Pi
  - Bouton **Redémarrer** (🔄 orange) pour redémarrage du système
  - Positionnés en bas à gauche de l'écran
  - Animations et effets visuels (hover, scale)
  - Confirmations de sécurité avant chaque action
- **Nouveaux endpoints API REST**
  - `POST /system/shutdown` - Arrête le Raspberry Pi (`shutdown -h now`)
  - `POST /system/reboot` - Redémarre le Raspberry Pi (`reboot`)
  - Délai de 2 secondes pour permettre l'envoi de la réponse HTTP
- **Documentation complète**
  - `SYSTEME_SHUTDOWN_REBOOT.md` - Guide de configuration sudo
  - Instructions pas à pas pour autoriser les commandes sans mot de passe
  - Section dépannage et sécurité
  - Conseils pour améliorer la sécurité avec un utilisateur dédié

### Modifié
- Interface web (static/index.html)
  - Ajout des fonctions JavaScript `confirmShutdown()` et `confirmReboot()`
  - Nouveaux styles CSS pour les boutons système
- Backend (src/main.rs)
  - Import de `std::process::Command` pour exécution des commandes système
  - Enregistrement des nouveaux endpoints dans le serveur HTTP

### Sécurité
- Les commandes système nécessitent une configuration sudo appropriée
- Confirmations doubles (dialogue de confirmation + message d'alerte)
- Documentation des bonnes pratiques de sécurité

## [1.2.0] - 2025-11-09

### Ajouté
- **Système complet de programmes d'entraînement personnalisés**
  - Structure `TrainingProgram` pour définir des programmes par intervalles
  - Intervalles configurables avec puissance cible et durée
  - Noms optionnels pour chaque intervalle
- **API REST complète pour la gestion des programmes**
  - `POST /program` - Créer un nouveau programme
  - `GET /programs` - Lister tous les programmes
  - `GET /program/{id}` - Obtenir un programme spécifique
  - `PUT /program/{id}` - Mettre à jour un programme
  - `DELETE /program/{id}` - Supprimer un programme
  - `POST /program/{id}/start` - Démarrer un programme
  - `POST /program/stop` - Arrêter le programme en cours
  - `GET /program/active` - Obtenir l'état du programme actif
- **Exécution automatique des programmes**
  - Changement de puissance en temps réel selon les intervalles
  - Suivi de la progression (pourcentage, temps écoulé)
  - Arrêt automatique en fin de programme
- **Interface web pour gérer les programmes**
  - `static/programs.html` - Page de gestion des programmes
  - Bouton d'accès dans l'interface principale (📋 Programmes)
  - Création, modification, suppression de programmes
  - Visualisation de la progression en temps réel
- **Scripts de démarrage automatique**
  - `autostart/startup-command.service` - Service systemd
  - `autostart/launch_terminal.sh` - Script de lancement
  - Documentation d'installation dans `autostart/README_installation.md`
- **Documentation**
  - `PROGRAMME_ENTRAINEMENT.md` - Guide complet du système de programmes

### Modifié
- **BikeController** (src/bike_controller.rs)
  - Ajout du stockage des programmes (`HashMap<String, TrainingProgram>`)
  - État d'exécution du programme actif (`ProgramExecutionState`)
  - Boucle de mise à jour toutes les secondes pour avancer dans le programme
  - Méthodes de gestion : create, update, delete, list, get, start, stop
- **Amélioration de la robustesse de connexion Bluetooth**
  - 5 tentatives de scan au lieu de 3
  - Backoff exponentiel entre les tentatives (2, 4, 8, 16 secondes)
  - Meilleurs messages de diagnostic
  - Nettoyage de l'ancienne connexion avant reconnexion
  - Délai de stabilisation après détection de l'appareil
  - Double tentative de connexion si la première échoue

### Fixé
- Gestion des erreurs de scan Bluetooth plus robuste
- Libération correcte des ressources Bluetooth lors de la reconnexion

## [1.1.0] - 2025-11-09

### Ajouté
- **Graphique d'évolution du RPM en temps réel**
  - Affichage visuel de l'historique des performances
  - Canvas avec tracé dynamique
  - Mise à jour automatique toutes les secondes

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
