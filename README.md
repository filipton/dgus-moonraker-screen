# DGUS Moonraker Screen
It's just overengineered software.

## Why?
If you have a creality `CR-10s (2020 probably)`, `ender 5 (pro or plus - idk which)` or `CP-01` printer and
you are using klipper - your lcd screen isn't working (yes i know there is a fork of klipper 
that just works, but then you can't use latest version of klipper). So this project is
making your screen great again!

## Other printers
Yes you can use it on other printer - but you must to buy a DWIN Display.
Comunication with display is done via UART connection with Raspberry PI
so your motherboard doesn't need to have display connector.

## Requirements
- Klipper
- Moonraker (you have it installed if you are using Mainsail/Fluidd)
- Working Micro SD Card
- Obviosly this screen

## Features (what can screen show/do)
- Current time
- Emergency stop
- Estimated print time
- Nozzle/Bed temp
- Print progress bar
- Pause/Resume/Stop print buttons
- Pre-heat screen (for now hardcoded - PLA 200/45)
- Basic toolhead movement

## Todo (near future)
- [x] Pause/Resume/Stop buttons in printing progress
- [x] Pre-heat screen
- [x] Toolhead movement
- [ ] Macros list (and ability to run them)
- [ ] Settings menu
