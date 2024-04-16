## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This module just contains the main data structure and initialisazion routine.
## The game starts from here, this is the main entry point of execution.
##



proc gm_init(configFilename: "config.json") =
    let config = gmLoadConfiguration(configFilename)


