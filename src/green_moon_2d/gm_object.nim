## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it uses Naylib (Raylib) internally.
##
## This module contains the code for graphic objects.
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
        custom*: JsonNode

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

proc gmSendMessageIntern(self: var GMObject, message: JsonNode): JsonNode =
    ## Intercepts message and performs default actions when given.
    result = newJNull()

    # TODO: implement

    if message.contains("addGroup"):
        discard
    elif message.contains("removeGroup"):
        discard
    elif message.contains("inGroup"):
        discard
    elif message.contains(""):
        discard
    else:
        discard

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

proc gmObjectAddGroup*(name: string, group: string) =
    ## Add the given object (by name) to the given group. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.groups.incl(group)
            return

    error_log(fmt("Can't add object {name} to group {group}, object not found!"), GMObjectNotFoundError)

proc gmObjectRemoveGroup*(name: string, group: string) =
    ## Removes the given object (by name) from the given group. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.groups.excl(group)
            return

    error_log(fmt("Can't remove object {name} to group {group}, object not found!"), GMObjectNotFoundError)

proc gmObjectInGroup*(name: string, group: string): bool =
    ## Returns true if the given object (by name) is in the group. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return group in ob.groups

    error_log(fmt("Can't check if object {name} is in group {group}, object not found!"), GMObjectNotFoundError)

proc gmObjectRemoveAllGroups*(group: string) =
    ## Removes all the objects from the given group.
    for ob in GMGlobObjects.objects.mitems():
        ob.groups.excl(group)

proc gmObjectSetUpdateOrder*(name: string, order: int32) =
    ## Sets the update order for the given object (by name). Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.updateOrder = order
            return

    error_log(fmt("Can't set update order for object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectGetUpdateOrder*(name: string): int32 =
    ## Returns the update order of the given object (by name). Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.updateOrder

    error_log(fmt("Can't get update order for object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectSetUpdateOrderGroup*(group: string, order: int32) =
    ## Sets the update order for all the objects in the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.updateOrder = order

proc gmObjectSetDrawOrder*(name: string, order: int32) =
    ## Sets the draw order for the given object (by name). Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.drawOrder = order
            return

    error_log(fmt("Can't set draw order for object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectGetDrawOrder*(name: string): int32 =
    ## Returns the draw order of the given object (by name). Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.drawOrder

    error_log(fmt("Can't get draw order for object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectSetDrawOrderGroup*(group: string, order: int32) =
    ## Sets the draw order for all the objects in the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.drawOrder = order

proc gmObjectSetActive*(name: string, active: bool = true) =
    ## Set active flag for given object (by name). Raise an exception if that name was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.active = active
            return

    error_log(fmt("Can't set active to object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectToggleActive*(name: string) =
    ## Toggle active flag for given object (by name). Raise an exception if that name was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.active = not ob.active
            return

    error_log(fmt("Can't toggle active to object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectGetActive*(name: string): bool =
    ## Returns if the given object (by name) is actie. Raise an exception of the object was nmot found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.active

    error_log(fmt("Can't get active state from object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectSetActiveGroup*(name: string, active: bool = true) =
    ## Set active flag for all objects that belong to the given group.
    for ob in GMGlobObjects.objects.mitems():
        if name in ob.groups:
            ob.active = active

proc gmObjectToggleActiveGroup*(name: string) =
    ## Toggle active flag for all objects that belong to the given group.
    for ob in GMGlobObjects.objects.mitems():
        if name in ob.groups:
            ob.active = not ob.active

proc gmObjectSetVisible*(name: string, visible: bool = true) =
    ## Set visible flag for given object (by name). Raise an exception if that name was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.visible = visible
            return

    error_log(fmt("Can't set visible to object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectToggleVisible*(name: string) =
    ## Toggle visible flag for given object (by name). Raise an exception if that name was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.visible = not ob.visible
            return

    error_log(fmt("Can't toggle visible to object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectGetVisible*(name: string): bool =
    ## Returns if the given object (by name) is visible. Raise an exception of the object was nmot found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.visible

    error_log(fmt("Can't get visible state from object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectSetVisibleGroup*(name: string, visible: bool = true) =
    ## Set visible flag for all objects that belong to the given group.
    for ob in GMGlobObjects.objects.mitems():
        if name in ob.groups:
            ob.visible = visible

proc gmObjectToggleVisibleGroup*(name: string) =
    ## Toggle visible flag for all objects that belong to the given group.
    for ob in GMGlobObjects.objects.mitems():
        if name in ob.groups:
            ob.visible = not ob.visible

proc gmObjectSendMessage*(name: string, message: JsonNode): JsonNode =
    ## Sends a message to the given object (by name). Raise an exception if that name was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.gmSendMessage(message)

    error_log(fmt("Can't send message to object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectSendMultiMessages*(name: string, messages: seq[JsonNode]): seq[JsonNode] =
    ## Sends multiple messages to the given object (by name). Raise an exception if that name was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            var r: seq[JsonNode] = @[]
            for m in messages:
                r.add(ob.gmSendMessage(m))
            return r

    error_log(fmt("Can't send multiple message to object {name}, not found!"), GMObjectNotFoundError)

proc gmObjectSendMessageGroup*(name: string, message: JsonNode): seq[(string, JsonNode)] =
    ## Send a message to all objects that belong to the given group.
    result = @[]

    for ob in GMGlobObjects.objects.mitems():
        if name in ob.groups:
            let v = ob.gmSendMessage(message)
            result.add((ob.name, v))

proc gmObjectSendMultiMessagesGroup*(name: string, messages: seq[JsonNode]): seq[(string, seq[JsonNode])] =
    ## Sends multiple messages to all objects that belong to the given group.
    result = @[]

    for ob in GMGlobObjects.objects.mitems():
        if name in ob.groups:
            var r: seq[JsonNode] = @[]
            for m in messages:
                r.add(ob.gmSendMessage(m))
            result.add((ob.name, r))

proc gmObjectSetCustomProperty*(name: string, property: JsonNode) =
    ## Sets a custom property for the object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.custom = property
            return

    error_log(fmt("Can't set custom property for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectGetCustomProperty*() =
    ## Gets a custom property for the object. Raise an exception if the object was not found.
    # TODO: implement
    discard


proc gmDrawObjects*() =
    ## Calls the draw() method for all visible objects, at each frame.
    GMGlobObjects.objects.sort do(a, b: GMObject) -> int:
        return cmp(a.drawOrder, b.drawOrder)

    for o in GMGlobObjects.objects.mitems():
        if o.visible:
            o.gmDraw()

proc gmUpdateObjects*() =
    ## Calls update() for all active objects, at each frame.
    GMGlobObjects.objects.sort do(a, b: GMObject) -> int:
        return cmp(a.updateOrder, b.updateOrder)

    for o in GMGlobObjects.objects.mitems():
        if o.active:
            o.gmUpdate()

