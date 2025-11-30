#!/bin/bash

# Script simple pour créer un point de sauvegarde (commit git)

if [ -z "$1" ]; then
    echo "Usage: ./checkpoint.sh \"Message de description\""
    echo "Exemple: ./checkpoint.sh \"Ajout du support tablette\""
    exit 1
fi

MESSAGE="$1"

# Ajouter tous les fichiers modifiés
git add .

# Créer le commit
git commit -m "$MESSAGE"

echo "✅ Point de sauvegarde créé avec succès !"
echo "Hash: $(git rev-parse --short HEAD)"
echo "Message: $MESSAGE"
