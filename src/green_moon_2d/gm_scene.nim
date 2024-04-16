## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This module contains the code that handles all the scenes.
##

type
    GMSceneManager* = object
        stuff: bool



proc gmInitSceneManager*(): GMSceneManager =
    result.stuff = true

