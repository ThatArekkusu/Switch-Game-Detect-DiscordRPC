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
- rust installed on system

In order to create your custom rich presence through `xorg-presence-rpc` please click the link for the respective repo in Credits and follow the instructions there, this is a prerequisite to this projects functionality

### Guide

1. Clone the repo and cd to the directory
```
git clone https://github.com/ThatArekkusu/Switch-Game-Detect-DiscordRPC
cd Switch-Game-Detect-DiscordRPC
```

2. Run with cargo
```
cargo run
```

### FAQ
**Q: What games are supported?**  
A: At the moment only support for mario kart world is added becuse games have to be hardcoded


**Q: What games are going to be supported?**  
A: I intend on adding support for Pokemon Pokopia, Tomodatchi Life Living the Dream, Splatoon 3, Mario Maker 2. These are games which i play so obviously these are the intended games to be implemented


**Q: How do i add more games myself?**  
A: In order to add games you must implement coordinates to screenshot a corrosponding are of the screen that can be mathced against a reference image and computate a similarity score then invoking that in the if statement, this is less complicated than it sounds but still arduous, i intend to add guides for this at some point


**Q: Does this work on windows?**  
A: At the moment this only works on linux, notably i use an arch distro so im confident in its functionality on that only, im unsure about debian based systems or otherwise but its possible it functions on that too, i hope to add windows support at some point in the future but im unsure about when that will be or the roadmap to make this possible


**Q: Is a capture card required?**  
A: Unfortunatly yes, this project relies completly on visual recognition in order to determine what game is being played, the previous project `nso-rpc` which is linked in the credits relied on switch online services API in order to retrieve game data but unfortunatly support for this has been stopped by Nintendo and it seems there isnt plans at current for this to change, as far as im aware this project is the only one which has any capability of passing your current played game through to a discord rich prescense


**Q: What is the future of this project?**  
A: I intend to add support for more games in the future if support for this project happens and games are requested


I also want to add furthur support for already implemented games in order to recognise screens other than title scerens in order to rectify the issue of going to home menu and returning back to the game not displaying the game as the title screen is the primary method of recognition at the moment


I hope to implement this for switch 1 and for lite mode theme soon, i feel this is rather easy and wont take long but as of current its not something which is supported


**footnote**  

Im still very inexperienced and am trying to avoid the traps of vibe coding as i learn, im nowhere near experienced enough to be confident in my abilities yet so updates may take some time please bare with me as i learn and improve with time, i hope to update this project semi-regularly especially if support for the project grows :3

### Credits

I will be attaching the projects which were both inspiration, requirements and a help to this project, thank you to all involved:
- [MCMI460] https://github.com/MCMi460/NSO-RPC/
- [thelinuxpirate] https://github.com/thelinuxpirate/xorg-discord-rpc