## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it uses Naylib (Raylib) internally.
##
## This module contains the code for normal objects and graphic objects.
##



type
    GMObjectManager = object
        stuff: bool

    GMGFXObjectManager = object
        stuff: bool

var GMGlobObjects: GMObjectManager
var GMGlobGFXObjects: GMGFXObjectManager

proc gmInitObjectManager*() =
    GMGlobObjects = GMObjectManager(stuff: true)

proc gmInitGFXObjectManager*() =
    GMGlobGFXObjects = GMGFXObjectManager(stuff: true)

