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

    if message.contains("addGroup"):
        let group = message["addGroup"].getStr()
        self.groups.incl(group)
    elif message.contains("removeGroup"):
        let group = message["removeGroup"].getStr()
        self.groups.excl(group)
    elif message.contains("inGroup"):
        let group = message["inGroup"].getStr()
        result = newJBool(group in self.groups)
    elif message.contains("setUpdateOrder"):
        self.updateOrder = int32(message["setUpdateOrder"].getInt())
    elif message.contains("getUpdateOrder"):
        result = newJInt(self.updateOrder)
    elif message.contains("setDrawOrder"):
        self.drawOrder = int32(message["setDrawOrder"].getInt())
    elif message.contains("getDrawOrder"):
        result = newJInt(self.drawOrder)
    elif message.contains("setActive"):
        self.active = message["setActive"].getBool() 
    elif message.contains("toggleActive"):
        self.active = not self.active
    elif message.contains("getActive"):
        result = newJBool(self.active)
    elif message.contains("setVisible"):
        self.visible = message["setVisible"].getBool()
    elif message.contains("toggleVisible"):
        self.visible = not self.visible
    elif message.contains("getVisible"):
        result = newJBool(self.visible)
    elif message.contains("setX"):
        self.x = float32(message["setX"].getFloat())
    elif message.contains("addX"):
        self.x += float32(message["addX"].getFloat())
    elif message.contains("getX"):
        result = newJFloat(self.x)
    elif message.contains("setY"):
        self.y = float32(message["setY"].getFloat())
    elif message.contains("addY"):
        self.y += float32(message["addY"].getFloat())
    elif message.contains("getY"):
        result = newJFloat(self.y)
    elif message.contains("setXY"):
        let elems = message["setXY"].getElems()
        self.x = float32(elems[0].getFloat())
        self.y = float32(elems[1].getFloat())
    elif message.contains("addXY"):
        let elems = message["getXY"].getElems()
        self.x += float32(elems[0].getFloat())
        self.y += float32(elems[1].getFloat())
    elif message.contains("getXY"):
        result = newJArray()
        result.add(newJFloat(self.x))
        result.add(newJFloat(self.y))
    elif message.contains("setProperty"):
        let elems = message["setProperty"].getElems()
        let name = elems[0].getStr()
        let property = elems[1]
        self.custom.add(name, property)
    elif message.contains("getProperty"):
        let name = message["getProperty"].getStr()
        if self.custom.contains(name):
            result = self.custom[name]
    elif message.contains("removeProperty"):
        let name = message["removeProperty"].getStr()
        if self.custom.contains(name):
            self.custom.delete(name)
    else:
        result = self.gmSendMessage(message)

proc gmFindObject*(name: string): Option[GMObject] =
    ## Return the object or none if no such object was found.
    for o in GMGlobObjects.objects.mitems():
        if o.name == name:
            return some(o)

    return none(GMObject)

proc gmInitObjectManager*() =
    ## Initializes the object manager
    GMGlobObjects = GMObjectManager(objects: @[])

proc gmAddObject*(name: string, newObject: GMObject) =
    ## Adds a new object by the given name. Raise an exception if that name is already in use.
    let ob = gmFindObject(name)

    if ob.isSome():
        error_log(fmt("Can't add object with name '{name}', it is already in use!"), GMObjectAlreadyExistsError)
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
        error_log(fmt("Can't remove '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectAddGroup*(name: string, group: string) =
    ## Add the given object to the given group. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.groups.incl(group)
            return

    error_log(fmt("Can't add '{name}' to group '{group}', object not found!"), GMObjectNotFoundError)

proc gmObjectRemoveGroup*(name: string, group: string) =
    ## Removes the given object from the given group. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.groups.excl(group)
            return

    error_log(fmt("Can't remove '{name}' from group '{group}', object not found!"), GMObjectNotFoundError)

proc gmObjectInGroup*(name: string, group: string): bool =
    ## Returns true if the given object is in the group. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return group in ob.groups

    error_log(fmt("Can't check if '{name}' is in group '{group}', object not found!"), GMObjectNotFoundError)

proc gmObjectRemoveAllGroups*(group: string) =
    ## Removes all the objects from the given group.
    for ob in GMGlobObjects.objects.mitems():
        ob.groups.excl(group)

proc gmObjectSetUpdateOrder*(name: string, order: int32) =
    ## Sets the update order for the given object . Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.updateOrder = order
            return

    error_log(fmt("Can't set update order for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectGetUpdateOrder*(name: string): int32 =
    ## Returns the update order of the given object . Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.updateOrder

    error_log(fmt("Can't get update order for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSetUpdateOrderGroup*(group: string, order: int32) =
    ## Sets the update order for all the objects in the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.updateOrder = order

proc gmObjectSetDrawOrder*(name: string, order: int32) =
    ## Sets the draw order for the given object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.drawOrder = order
            return

    error_log(fmt("Can't set draw order for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectGetDrawOrder*(name: string): int32 =
    ## Returns the draw order of the given object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.drawOrder

    error_log(fmt("Can't get draw order for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSetDrawOrderGroup*(group: string, order: int32) =
    ## Sets the draw order for all the objects in the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.drawOrder = order

proc gmObjectSetActive*(name: string, active: bool = true) =
    ## Set active flag for given object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.active = active
            return

    error_log(fmt("Can't set active flag for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectToggleActive*(name: string) =
    ## Toggle active flag for given object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.active = not ob.active
            return

    error_log(fmt("Can't toggle active flag for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectGetActive*(name: string): bool =
    ## Returns the given object active flag. Raise an exception of the object was nmot found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.active

    error_log(fmt("Can't get active flag from '{name}', object not found!"), GMObjectNotFoundError)

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
    ## Set visible flag for given object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.visible = visible
            return

    error_log(fmt("Can't set visible flag for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectToggleVisible*(name: string) =
    ## Toggle visible flag for given object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.visible = not ob.visible
            return

    error_log(fmt("Can't toggle visible flag for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectGetVisible*(name: string): bool =
    ## Returns the visibility flag for the given object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.visible

    error_log(fmt("Can't get visible flag from '{name}', object not found!"), GMObjectNotFoundError)

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
    ## Sends a message to the given object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            return ob.gmSendMessageIntern(message)

    error_log(fmt("Can't send message to '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSendMultiMessages*(name: string, messages: seq[JsonNode]): seq[JsonNode] =
    ## Sends multiple messages to the given object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            var r: seq[JsonNode] = @[]
            for m in messages:
                r.add(ob.gmSendMessageIntern(m))
            return r

    error_log(fmt("Can't send multiple message to '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSendMessageGroup*(name: string, message: JsonNode): seq[(string, JsonNode)] =
    ## Send a message to all objects that belong to the given group.
    result = @[]

    for ob in GMGlobObjects.objects.mitems():
        if name in ob.groups:
            let v = ob.gmSendMessageIntern(message)
            result.add((ob.name, v))

proc gmObjectSendMultiMessagesGroup*(name: string, messages: seq[JsonNode]): seq[(string, seq[JsonNode])] =
    ## Sends multiple messages to all objects that belong to the given group.
    result = @[]

    for ob in GMGlobObjects.objects.mitems():
        if name in ob.groups:
            var r: seq[JsonNode] = @[]
            for m in messages:
                r.add(ob.gmSendMessageIntern(m))
            result.add((ob.name, r))

proc gmObjectSetCustomProperty*(name: string, property: string, value: JsonNode) =
    ## Sets a custom property for the object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            ob.custom[property] = value
            return

    error_log(fmt("Can't set custom property for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectGetCustomProperty*(name: string, property: string): JsonNode =
    ## Gets a custom property for the object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            if ob.custom.contains(property):
                return ob.custom[property]
            else:
                return newJNull()

    error_log(fmt("Can't get custom property for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectRemoveCustomProperty*(name: string, property: string) =
    ## Removes a custom property for the object. Raise an exception if the object was not found.
    for ob in GMGlobObjects.objects.mitems():
        if ob.name == name:
            if ob.custom.contains(property):
                ob.custom.delete(property)

    error_log(fmt("Can't remove custom property for '{name}', object not found!"), GMObjectNotFoundError)

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

