#!/bin/bash
# Script to open a terminal and launch the Rust server at startup
# Note: This script is optional - the systemd service runs without GUI terminal

# Command: navigate to project directory and launch cargo run --release
COMMAND="cd $HOME/Documents/SkylonRemoteApp && $HOME/.cargo/bin/cargo run --release"

# Open LXTerminal (default terminal on Raspberry Pi OS)
# and execute the command
lxterminal -e bash -c "$COMMAND; exec bash"

# Alternatives for other terminals:
# xterm -hold -e "$COMMAND"
# gnome-terminal -- bash -c "$COMMAND; exec bash"
