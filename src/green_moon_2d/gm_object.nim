## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it uses Naylib (Raylib) internally.
##
## This module contains the code for normal objects and graphic objects.
##


# Nim std imports
import std/sets

type
    GMObject* = ref object of RootObj
        name: string
        groups: HashSet[string]
        order: int32

    GMGFXObject* = ref object of RootObj
        name: string
        groups: HashSet[string]
        order: int32
        x: float32
        y: float32

    GMObjectManager = object
        objects: seq[GMObject]

    GMGFXObjectManager = object
        objects: seq[GMObject]

var GMGlobObjects: GMObjectManager
var GMGlobGFXObjects: GMGFXObjectManager

# GMGFXObject:
proc gmDraw*(self: var GMGFXObject) =
    discard

proc gmUpdate*(self: var GMGFXObject) =
    discard

# GMObject:
proc gmUpdate*(self: var GMObject) =
    discard

# GMGFXObjectManager:
proc gmInitObjectManager*() =
    GMGlobObjects = GMObjectManager(objects: @[])

proc gmDrawGFXObjects*() =
    discard

proc gmUpdateGFXObjects*() =
    discard

# GMObjectManager:
proc gmInitGFXObjectManager*() =
    GMGlobGFXObjects = GMGFXObjectManager(objects: @[])

proc gmUpdateObjects*() =
    discard



