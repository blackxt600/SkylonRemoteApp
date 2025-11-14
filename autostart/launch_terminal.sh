#!/bin/bash
# Script pour ouvrir un terminal et lancer le serveur Rust au démarrage

# Commande : se déplacer dans le dossier et lancer cargo run --release
COMMAND="cd /home/skylon/Documents/SkylonRemoteApp && /home/skylon/.cargo/bin/cargo run --release"

# Ouvre un terminal LXTerminal (terminal par défaut sur Raspberry Pi OS)
# et exécute la commande
lxterminal -e bash -c "$COMMAND; exec bash"

# Alternatives pour d'autres terminaux :
# xterm -hold -e "$COMMAND"
# gnome-terminal -- bash -c "$COMMAND; exec bash"
