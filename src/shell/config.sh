#!/bin/bash
CONFIG=/home/$USER/Switch-2-GameDetect-RPC/temp/config.toml
#testing
echo "test"
echo $CURRENT_GAME

PREVIOUS_GAME=$(grep 'name =' /home/$USER/Switch-2-GameDetect-RPC/temp/config.toml | head -1 | awk -F"'" '{print $2}')
echo $PREVIOUS_GAME


sed -i "s|name = '$PREVIOUS_GAME'|name = '$CURRENT_GAME'|g" $CONFIG