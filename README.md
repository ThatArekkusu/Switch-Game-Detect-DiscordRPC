WIP

This project aims to fix my issue of my discord custom rich presence on linux not displaying what game im playing

# IMPORTANT

if your `/home/{YourUserHere}/.config/xorg-presence/config.toml` has text following an `=` and its in quotations, Ex: `name="mario"` THIS WILL BREAK, ensure your config file is using quotations, Ex: `name='mario'`, this is because `sed` the command which replaces text within files which is whats editing the `/home/{YourUserHere}/.config/xorg-presence/config.toml` file to make it display the game you're playing uses quotations in the command and it will break if your text also uses quotations, ALSO ensure there is a between = spaces such as `name = 'mario'` or it also breaks sed

## Explanation

There exists a project called `NSO-RPC` by MCMI460 which relied on your nintendo session token and the nintendo API in order to pass through the game your currently playing through to discord however this no longer works as Nintendo has removed this service, my project aims to rectify this by using visual detection to detect what game you're playing and pass it along that way, this project is intended for me personally so games supported will be those which i play however it should not be difficult to manually import what you wish to detect also

## Requirements

ALL LINKS IN CREDITS

I personally use `xorg-presence` in order to manually start a discord rich presence which detects my `ffplay` window which inputs from my elgato capture card to play my nintendo switch 2 making it appear as if its detecting my switch 2 automatically however its just detecting the input window, i then use `pw-link` to pipe the audio directly to my DAC and disconnect it through a shell script, all of my personal files for this will be attached in the `personal` folder within the root directory of this project

Requirements
- Nintendo Switch 1/2 Running through a capture card into your window capture of choice
- `xorg-presence-rpc` (Despite being `xorg` it does work on wayland with some modifications just as i do it)
- a config file for `xorg-presence-rpc` stored at `/home/{YourUserHere}/.config/xorg-presence/config.toml`, this is the file the project will edit once a game has been visually detected in order to create the custom rich presense for your specific game of choice
- a shell file in order to start your input capture window and subsequently the project, personally mine runs through a `.desktop` entry ive created which again will be attached in the `personal` folder in the root directory of this project
- a discord application from the Discord Developer Site

## Prerequisits

- A linux distrubution which supports xorg/wayland (i use arch btw)

In order to create your custom rich presence through `xorg-presence-rpc` please click the link for the respective repo in Credits and follow the instructions there, this is a prerequisite to this projects functionality

### Credits

I will be attaching the projects which were both inspiration, requirements and a help to this project, thank you to all involved:
- [MCMI460] https://github.com/MCMi460/NSO-RPC/
- [thelinuxpirate] https://github.com/thelinuxpirate/xorg-discord-rpc

### Guide