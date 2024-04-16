## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This module contains the code for normal objects and graphic objects.
##



type
    GMObjectManager* = object
        stuff: bool

    GMGFXObjectManager* = object
        stuff: bool



proc gmInitObjectManager*(): GMObjectManager =
    result.stuff = true

proc gmInitGFXObjectManager*(): GMGFXObjectManager =
    result.stuff = true


