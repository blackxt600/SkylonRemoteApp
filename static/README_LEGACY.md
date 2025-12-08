# Version Legacy pour Anciens Navigateurs

## À propos

Le fichier `index-legacy.html` est une version simplifiée et optimisée pour les anciens navigateurs et appareils, notamment :

- **iPad ancienne génération** avec Chrome 63 ou Safari ancien
- **Écrans 10 pouces** et moins
- **Appareils avec performances limitées**

## Accès

Pour accéder à cette version, ouvrez dans votre navigateur :
```
http://[IP-RASPBERRY]:8080/index-legacy.html
```

## Différences avec la version standard

### ✅ Fonctionnalités conservées

- 8 programmes d'entraînement (Plat, Escalier, Vallée, Collines, Montagne, Col Alpin, Intervalle, Pyramide)
- Contrôle manuel de la puissance (25-400W)
- Ajustement de la difficulté (+/- 100W)
- Histogramme visuel avec progression en temps réel
- Graphique RPM avec ligne cible
- Timer avec démarrage/pause automatique
- Boutons système (Éteindre/Redémarrer)
- Mode plein écran

### ❌ Fonctionnalités retirées

- **Mode Jeu (Space Runner)** - trop gourmand en ressources
- **Programme personnalisé "Sur mesure"** - interface d'édition complexe
- **Effets visuels avancés** :
  - Glassmorphism
  - Backdrop-filter
  - Animations de gradient
  - Particules et effets visuels

### 🔧 Optimisations techniques

#### JavaScript
- **Compatibilité ES5** :
  - Utilisation de `var` au lieu de `const/let`
  - Fonctions traditionnelles au lieu de arrow functions
  - `.then()/.catch()` au lieu de `async/await`
  - Boucles `for` classiques
  - Pas de spread operator

#### CSS
- **Flexbox uniquement** (pas de CSS Grid)
- **Pas de backdrop-filter** (non supporté Chrome 63)
- **Prefixes vendor** pour compatibilité maximale (-webkit-)
- **Dégradés simplifiés**
- **Transitions basiques**

#### Rendu visuel
- **Histogramme sur Canvas** :
  - Remplacement de l'histogramme Flexbox par un Canvas HTML5
  - Meilleure compatibilité avec anciens navigateurs
  - Rendu plus fiable et performant
  - Barres colorées avec coins arrondis
  - Grille et échelle intégrées

#### Interface
- **Layout 2 colonnes** au lieu de 3 (économie d'espace)
- **Programmes en grille horizontale** (4 par ligne)
- **Tailles réduites** :
  - Padding et marges diminués
  - Polices optimisées
  - Boutons compacts
- **Design responsive** pour petits écrans

## Performances

Cette version est **significativement plus légère** :
- Moins de JavaScript (~40% de code en moins)
- CSS simplifié (pas d'effets complexes)
- Pas de canvas pour jeu
- Meilleure fluidité sur anciens appareils

## Compatibilité navigateur

Testé et optimisé pour :
- ✅ Chrome 63+ (décembre 2017)
- ✅ Safari iOS 10+
- ✅ Firefox 57+
- ✅ Edge 16+

## Recommandations

### Pour iPad ancien (10 pouces)
1. Utilisez le **mode paysage** (horizontal)
2. Activez le **mode plein écran** (bouton ⛶ en haut à gauche)
3. Fermez les autres onglets pour libérer la mémoire
4. Désactivez les mises à jour automatiques pendant l'utilisation

### Si vous rencontrez des problèmes
1. Videz le cache du navigateur
2. Redémarrez l'application Safari/Chrome
3. Vérifiez que JavaScript est activé
4. Assurez-vous d'une bonne connexion WiFi

## Passage d'une version à l'autre

Vous pouvez basculer entre les deux versions à tout moment :

- **Version standard** : `http://[IP]:8080/` ou `http://[IP]:8080/index.html`
- **Version legacy** : `http://[IP]:8080/index-legacy.html`

Les deux versions communiquent avec le même serveur backend, donc vos entraînements sont synchronisés.

## Support

Pour toute question ou problème avec cette version legacy :
1. Vérifiez que votre navigateur est à jour (Chrome 63 minimum)
2. Consultez la console JavaScript (F12) pour les erreurs
3. Testez d'abord avec la version standard pour isoler le problème

---

**Version créée le :** 2025-12-08
**Compatible avec :** Chrome 63+, Safari iOS 10+, écrans 10 pouces et moins
