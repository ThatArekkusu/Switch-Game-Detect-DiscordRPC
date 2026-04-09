#!/bin/bash
pkill -f xorg-discord-rpc
sleep 0.2

nohup xorg-discord-rpc -i 1488046634452521002 -l /home/kusu/.config/xorg-presence/config.toml > /dev/null 2>&1 &
disown