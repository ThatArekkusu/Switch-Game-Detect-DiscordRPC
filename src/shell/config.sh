#!/bin/bash
CONFIG=~/.config/xorg-presence/config.toml

PREVIOUS_GAME=$(grep "name[[:space:]]*=[[:space:]]*'" "$CONFIG" | head -1 | awk -F"'" '{print $2}')

echo "Detected: $PREVIOUS_GAME"

if [ -z "$PREVIOUS_GAME" ]; then
    echo "Error: Could not extract the game name."
    exit 1
fi

sed -i "s|name = '$PREVIOUS_GAME'|name = '$CURRENT_GAME'|g" "$CONFIG"

