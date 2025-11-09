# Gestion des Programmes d'Entraînement

Cette fonctionnalité vous permet de créer des programmes d'entraînement personnalisés composés d'intervalles avec différents niveaux de puissance et durées.

## Structure d'un Programme

Un programme d'entraînement est composé de :
- **id** : Identifiant unique du programme
- **name** : Nom du programme
- **description** : Description optionnelle
- **intervals** : Liste d'intervalles, chacun avec :
  - **duration_secs** : Durée en secondes
  - **power_target** : Puissance cible en watts (0-400W)
  - **name** : Nom optionnel de l'intervalle (ex: "Échauffement", "Sprint", "Récupération")

## Endpoints API

### Créer un programme
```bash
POST /program
Content-Type: application/json

{
  "id": "interval-training",
  "name": "Entraînement par intervalles",
  "description": "Programme HIIT de 20 minutes",
  "intervals": [
    {
      "duration_secs": 300,
      "power_target": 80,
      "name": "Échauffement"
    },
    {
      "duration_secs": 60,
      "power_target": 200,
      "name": "Sprint"
    },
    {
      "duration_secs": 120,
      "power_target": 100,
      "name": "Récupération"
    },
    {
      "duration_secs": 60,
      "power_target": 200,
      "name": "Sprint"
    },
    {
      "duration_secs": 120,
      "power_target": 100,
      "name": "Récupération"
    },
    {
      "duration_secs": 60,
      "power_target": 200,
      "name": "Sprint"
    },
    {
      "duration_secs": 300,
      "power_target": 60,
      "name": "Retour au calme"
    }
  ]
}
```

### Lister tous les programmes
```bash
GET /programs
```

### Obtenir un programme spécifique
```bash
GET /program/{id}
```

### Mettre à jour un programme
```bash
PUT /program/{id}
Content-Type: application/json

{
  "id": "interval-training",
  "name": "Entraînement par intervalles (modifié)",
  "description": "Programme HIIT de 20 minutes - version 2",
  "intervals": [...]
}
```

### Supprimer un programme
```bash
DELETE /program/{id}
```

### Démarrer un programme
```bash
POST /program/{id}/start
```

### Arrêter le programme en cours
```bash
POST /program/stop
```

### Obtenir l'état du programme actif
```bash
GET /program/active
```

Retourne :
```json
{
  "program_id": "interval-training",
  "program_name": "Entraînement par intervalles",
  "current_interval_index": 2,
  "elapsed_in_interval": 45,
  "total_elapsed": 405,
  "total_duration": 1020,
  "current_power_target": 100,
  "current_interval_name": "Récupération",
  "program": {...}
}
```

## Exemples d'utilisation avec curl

### 1. Créer un programme d'échauffement simple
```bash
curl -X POST http://localhost:8080/program \
  -H "Content-Type: application/json" \
  -d '{
    "id": "warmup",
    "name": "Échauffement",
    "description": "Programme d'\''échauffement de 10 minutes",
    "intervals": [
      {
        "duration_secs": 180,
        "power_target": 50,
        "name": "Phase 1"
      },
      {
        "duration_secs": 240,
        "power_target": 80,
        "name": "Phase 2"
      },
      {
        "duration_secs": 180,
        "power_target": 100,
        "name": "Phase 3"
      }
    ]
  }'
```

### 2. Créer un programme HIIT avancé
```bash
curl -X POST http://localhost:8080/program \
  -H "Content-Type: application/json" \
  -d '{
    "id": "hiit-advanced",
    "name": "HIIT Avancé",
    "description": "Programme intensif 30 minutes",
    "intervals": [
      {
        "duration_secs": 300,
        "power_target": 80,
        "name": "Échauffement"
      },
      {
        "duration_secs": 45,
        "power_target": 250,
        "name": "Sprint 1"
      },
      {
        "duration_secs": 90,
        "power_target": 90,
        "name": "Récupération 1"
      },
      {
        "duration_secs": 45,
        "power_target": 250,
        "name": "Sprint 2"
      },
      {
        "duration_secs": 90,
        "power_target": 90,
        "name": "Récupération 2"
      },
      {
        "duration_secs": 45,
        "power_target": 250,
        "name": "Sprint 3"
      },
      {
        "duration_secs": 90,
        "power_target": 90,
        "name": "Récupération 3"
      },
      {
        "duration_secs": 45,
        "power_target": 250,
        "name": "Sprint 4"
      },
      {
        "duration_secs": 90,
        "power_target": 90,
        "name": "Récupération 4"
      },
      {
        "duration_secs": 300,
        "power_target": 60,
        "name": "Retour au calme"
      }
    ]
  }'
```

### 3. Créer un programme d'endurance
```bash
curl -X POST http://localhost:8080/program \
  -H "Content-Type: application/json" \
  -d '{
    "id": "endurance-45min",
    "name": "Endurance 45 min",
    "description": "Programme d'\''endurance à intensité progressive",
    "intervals": [
      {
        "duration_secs": 600,
        "power_target": 100,
        "name": "Échauffement"
      },
      {
        "duration_secs": 900,
        "power_target": 140,
        "name": "Phase montée"
      },
      {
        "duration_secs": 600,
        "power_target": 160,
        "name": "Phase plateau"
      },
      {
        "duration_secs": 600,
        "power_target": 120,
        "name": "Phase descente"
      }
    ]
  }'
```

### 4. Lister tous les programmes
```bash
curl http://localhost:8080/programs
```

### 5. Démarrer un programme
```bash
curl -X POST http://localhost:8080/program/hiit-advanced/start
```

### 6. Vérifier l'état du programme en cours
```bash
curl http://localhost:8080/program/active
```

### 7. Arrêter le programme
```bash
curl -X POST http://localhost:8080/program/stop
```

## Comportement du système

- **Changement automatique d'intervalle** : Le système change automatiquement de puissance lorsqu'un intervalle est terminé
- **Logs informatifs** : Le serveur affiche des messages pour chaque changement d'intervalle
- **Sécurité** :
  - Impossible de démarrer un programme si un autre est en cours
  - Impossible de modifier ou supprimer un programme en cours d'exécution
  - Validation des valeurs (puissance max 400W, durées positives)
- **Arrêt automatique** : Le programme s'arrête automatiquement à la fin

## Interface utilisateur

Vous pouvez créer une interface HTML/JavaScript pour :
- Afficher la liste des programmes avec des cartes
- Créer/éditer des programmes avec des sliders pour chaque intervalle
- Visualiser la progression en temps réel avec une barre de progression
- Contrôler le démarrage/arrêt des programmes
- Afficher l'intervalle actuel et le temps restant

### Exemple de structure de slider par intervalle

```html
<div class="interval-editor">
  <h3>Intervalle 1</h3>
  <label>Nom: <input type="text" id="interval-1-name" value="Échauffement"></label>
  <label>Durée (secondes): <input type="range" id="interval-1-duration" min="30" max="600" value="300"></label>
  <label>Puissance (watts): <input type="range" id="interval-1-power" min="0" max="400" value="80"></label>
  <button onclick="removeInterval(1)">Supprimer</button>
</div>
```

## Notes techniques

- Les programmes sont stockés en mémoire (non persistants entre les redémarrages)
- Un seul programme peut être exécuté à la fois
- Le serveur met à jour la puissance toutes les secondes
- Les changements de puissance sont appliqués automatiquement au vélo
