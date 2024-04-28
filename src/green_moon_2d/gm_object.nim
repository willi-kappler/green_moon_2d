## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it uses Naylib (Raylib) internally.
##
## This module contains the code for normal objects and graphic objects.
##


# Nim std imports
import std/json
import std/sets
import std/options
import std/algorithm

from std/strformat import fmt

# Local imports
import gm_log

type
    GMObject* = ref object of RootObj
        name: string
        groups*: HashSet[string]
        updateOrder*: int32
        drawOrder*: int32
        active*: bool
        visible*: bool
        x*: float32
        y*: float32

    GMObjectManager = object
        objects: seq[GMObject]

    GMObjectNotFoundError* = object of CatchableError
    GMObjectAlreadyExistsError* = object of CatchableError

var GMGlobObjects: GMObjectManager

# GMObject:
method gmDraw*(self: var GMObject) {.base.} =
    ## This method can be implemented in order to draw the object.
    discard

method gmUpdate*(self: var GMObject) {.base.} =
    ## This method can be implemented in order to update the object.
    discard

method gmSendMessage*(self: var GMObject, message: JsonNode): JsonNode {.base.} =
    ## This method can be implemented in order to send a custom message to the object.
    return newJNull()

proc gmFindObject*(name: string): Option[GMObject] =
    ## Return the object (by name) or none if no such object was found.
    for o in GMGlobObjects.objects.mitems():
        if o.name == name:
            return some(o)

    return none(GMObject)

# GMObjectManager:
proc gmInitObjectManager*() =
    ## Initializes the object manager
    GMGlobObjects = GMObjectManager(objects: @[])

proc gmAddObject*(name: string, newObject: GMObject) =
    ## Adds a new object by the given name. Raise an exception if that name is already in use.
    let ob = gmFindObject(name)

    if ob.isSome():
        error_log(fmt("Can't add object {name}, it already exists!"), GMObjectAlreadyExistsError)
    else:
        GMGlobObjects.objects.add(newObject)

proc gmDeleteObject*(name: string) =
    ## Deletes an object with the given name. Raise an exception if the object was not found.
    var idx = none(uint32)

    for (i, ob) in GMGlobObjects.objects.pairs():
        if ob.name == name:
            idx = some(uint32(i))
            break

    if idx.isSome():
        let i = idx.get()
        GMGlobObjects.objects.del(i)
    else:
        error_log(fmt("Can't remove object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectSendMessage*(name: string, message: JsonNode): JsonNode =
    ## Sends a message to the given object (by name). Raise an exception if that name was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.gmSendMessage(message)

    error_log(fmt("Can't send message to object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectSendMessageGroup*(name: string, message: JsonNode): seq[(string, JsonNode)] =
    ## Send a message to all objects that belong to the given group.
    result = @[]

    for ob in GMGlobObjects.objects.mitems():
        if name in ob.groups:
            let v = ob.gmSendMessage(message)
            result.add((ob.name, v))

proc gmDrawObjects*() =
    ## Calls the draw() method for all visible objects, at each frame.
    GMGlobObjects.objects.sort do(a, b: GMObject) -> int:
        return cmp(a.drawOrder, b.drawOrder)

    for o in GMGlobObjects.objects.mitems():
        if o.visible:
            o.gmDraw()

proc gmUpdateObjects*() =
    ## Calls update() for all active (non-gfx) objects, at each frame.
    GMGlobObjects.objects.sort do(a, b: GMObject) -> int:
        return cmp(a.updateOrder, b.updateOrder)

    for o in GMGlobObjects.objects.mitems():
        if o.active:
            o.gmUpdate()



