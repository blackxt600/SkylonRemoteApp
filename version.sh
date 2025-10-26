#!/bin/bash

# Script de gestion des versions
# Usage: ./version.sh [patch|minor|major] "Description du changement"

set -e

if [ $# -lt 2 ]; then
    echo "Usage: $0 [patch|minor|major] \"Description du changement\""
    echo ""
    echo "Exemples:"
    echo "  $0 patch \"Correction du bug de connexion\""
    echo "  $0 minor \"Ajout du mode nuit\""
    echo "  $0 major \"Refonte complète de l'API\""
    exit 1
fi

TYPE=$1
DESCRIPTION=$2

# Lire la version actuelle
CURRENT_VERSION=$(cat VERSION)
echo "Version actuelle: $CURRENT_VERSION"

# Découper la version
IFS='.' read -ra VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR=${VERSION_PARTS[0]}
MINOR=${VERSION_PARTS[1]}
PATCH=${VERSION_PARTS[2]}

# Calculer la nouvelle version
case $TYPE in
    patch)
        PATCH=$((PATCH + 1))
        ;;
    minor)
        MINOR=$((MINOR + 1))
        PATCH=0
        ;;
    major)
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        ;;
    *)
        echo "Type invalide. Utilisez: patch, minor, ou major"
        exit 1
        ;;
esac

NEW_VERSION="$MAJOR.$MINOR.$PATCH"
echo "Nouvelle version: $NEW_VERSION"

# Demander confirmation
read -p "Continuer avec v$NEW_VERSION? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Annulé."
    exit 1
fi

# Mettre à jour VERSION
echo "$NEW_VERSION" > VERSION

# Ajouter l'entrée dans CHANGELOG.md
DATE=$(date +%Y-%m-%d)
CHANGELOG_ENTRY="## [$NEW_VERSION] - $DATE\n\n### Modifié\n- $DESCRIPTION\n\n"

# Insérer après la ligne ## [Unreleased] ou après le premier ##
sed -i "/^## \[/a\\$CHANGELOG_ENTRY" CHANGELOG.md 2>/dev/null || \
    echo -e "$CHANGELOG_ENTRY$(cat CHANGELOG.md)" > CHANGELOG.md

echo "✓ VERSION mis à jour"
echo "✓ CHANGELOG.md mis à jour"

# Commit et tag
git add VERSION CHANGELOG.md
git commit -m "Release v$NEW_VERSION - $DESCRIPTION"
git tag -a "v$NEW_VERSION" -m "Version $NEW_VERSION - $DESCRIPTION"

echo "✓ Commit créé"
echo "✓ Tag v$NEW_VERSION créé"
echo ""
echo "Pour pousser vers le dépôt distant:"
echo "  git push origin master --tags"
