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
import gm_json
import gm_math

type
    GMObject* = ref object of RootObj
        name: string
        groups*: HashSet[string]
        updateOrder*: int32
        drawOrder*: int32
        active*: bool
        visible*: bool
        receiveBaseMessage*: bool
        position*: GMVec2D
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

proc gmGetName*(self: GMObject): string =
    return self.name

proc gmSendMessageIntern(self: var GMObject, message: JsonNode): JsonNode =
    ## Intercepts message and performs default actions when given.
    result = newJNull()

    if self.receiveBaseMessage:
        return self.gmSendMessage(message)

    if message.contains("addGroup"):
        self.groups.incl(gmGetString(message, "addGroup"))
    elif message.contains("removeGroup"):
        self.groups.excl(gmGetString(message, "removeGroup"))
    elif message.contains("inGroup"):
        let group = gmGetString(message, "inGroup")
        result = newJBool(group in self.groups)
    elif message.contains("setUpdateOrder"):
        self.updateOrder = gmGetInt32(message, "setUpdateOrder")
    elif message.contains("getUpdateOrder"):
        result = newJInt(self.updateOrder)
    elif message.contains("setDrawOrder"):
        self.drawOrder = gmGetInt32(message, "setDrawOrder")
    elif message.contains("getDrawOrder"):
        result = newJInt(self.drawOrder)
    elif message.contains("setActive"):
        self.active = gmGetBool(message, "setActive")
    elif message.contains("toggleActive"):
        self.active = not self.active
    elif message.contains("getActive"):
        result = newJBool(self.active)
    elif message.contains("setVisible"):
        self.visible = gmGetBool(message, "setVisible")
    elif message.contains("toggleVisible"):
        self.visible = not self.visible
    elif message.contains("getVisible"):
        result = newJBool(self.visible)
    elif message.contains("setX"):
        self.position.x = gmGetFloat32(message, "setX")
    elif message.contains("addX"):
        self.position.x += gmGetFloat32(message, "addX")
    elif message.contains("getX"):
        result = newJFloat(self.position.x)
    elif message.contains("setY"):
        self.position.y = gmGetFloat32(message, "setY")
    elif message.contains("addY"):
        self.position.y += gmGetFloat32(message, "addY")
    elif message.contains("getY"):
        result = newJFloat(self.position.y)
    elif message.contains("setXY"):
        (self.position.x, self.position.y) = gmGetF32F32(message, "setXY")
    elif message.contains("addXY"):
        let (x, y) = gmGetF32F32(message, "addXY")
        self.position.x += x
        self.position.y += y
    elif message.contains("getXY"):
        result = newJArray()
        result.add(newJFloat(self.position.x))
        result.add(newJFloat(self.position.y))
    elif message.contains("setProperty"):
        let elems = gmGetNodes(message, "setProperty")
        let name = elems[0].getStr()
        let property = elems[1]
        self.custom[name] = property
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

proc gmGetIndex(name: string): Option[uint32] =
    for i in 0..GMGlobObjects.objects.high():
        if GMGlobObjects.objects[i].name == name:
            return some(uint32(i))

    return none(uint32)

proc gmInitObjectManager*() =
    ## Initializes the object manager
    GMGlobObjects = GMObjectManager(objects: @[])

proc gmAddObject*(name: string, newObject: var GMObject) =
    ## Adds a new object by the given name. Raise an exception if that name is already in use.
    let ob = gmFindObject(name)

    if ob.isSome():
        error_log(fmt("Can't add object with name '{name}', it is already in use!"), GMObjectAlreadyExistsError)
    else:
        newObject.name = name
        newObject.groups = initHashSet[string]()
        newObject.custom = newJNull()
        GMGlobObjects.objects.add(newObject)

proc gmDeleteObject*(name: string) =
    ## Deletes an object with the given name. Raise an exception if the object was not found.
#    var idx = none(uint32)
#
#    for (i, ob) in GMGlobObjects.objects.pairs():
#        if ob.name == name:
#            idx = some(uint32(i))
#            break

    let idx = gmGetIndex(name)

    if idx.isSome():
        let i = idx.get()
        GMGlobObjects.objects.del(i)
    else:
        error_log(fmt("Can't remove '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectAddGroup*(name: string, group: string) =
    ## Add the given object to the given group. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            ob.groups.incl(group)
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        obj.groups.incl(group)
    else:
        error_log(fmt("Can't add '{name}' to group '{group}', object not found!"), GMObjectNotFoundError)

proc gmObjectAddGroups*(name: string, groups: openArray[string]) =
    ## Add the given object to the given groups. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            for g in groups:
#                ob.groups.incl(g)
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        for g in groups:
            obj.groups.incl(g)
    else:
        error_log(fmt("Can't add '{name}' to groups '{groups}', object not found!"), GMObjectNotFoundError)

proc gmObjectRemoveGroup*(name: string, group: string) =
    ## Removes the given object from the given group. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            ob.groups.excl(group)
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        obj.groups.excl(group)
    else:
        error_log(fmt("Can't remove '{name}' from group '{group}', object not found!"), GMObjectNotFoundError)

proc gmObjectRemoveGroups*(name: string, groups: openArray[string]) =
    ## Removes the given object from the given groups. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            for g in groups:
#                ob.groups.excl(g)
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        for g in groups:
            obj.groups.excl(g)
    else:
        error_log(fmt("Can't remove '{name}' from groups '{groups}', object not found!"), GMObjectNotFoundError)

proc gmObjectInGroup*(name: string, group: string): bool =
    ## Returns true if the given object is in the group. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            return group in ob.groups

    let ob = gmFindObject(name)

    if ob.isSome():
        let obj = ob.get()
        return group in obj.groups
    else:
        error_log(fmt("Can't check if '{name}' is in group '{group}', object not found!"), GMObjectNotFoundError)

proc gmObjectRemoveGroupFromAll*(group: string) =
    ## Removes all the objects from the given group.
    for ob in GMGlobObjects.objects.mitems():
        ob.groups.excl(group)

proc gmObjectSetUpdateOrder*(name: string, order: int32) =
    ## Sets the update order for the given object . Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            ob.updateOrder = order
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        obj.updateOrder = order
    else:
        error_log(fmt("Can't set update order for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectGetUpdateOrder*(name: string): int32 =
    ## Returns the update order of the given object . Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            return ob.updateOrder

    let ob = gmFindObject(name)

    if ob.isSome():
        let obj = ob.get()
        return obj.updateOrder
    else:
        error_log(fmt("Can't get update order for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSetUpdateOrderGroup*(group: string, order: int32) =
    ## Sets the update order for all the objects in the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.updateOrder = order

proc gmObjectSetDrawOrder*(name: string, order: int32) =
    ## Sets the draw order for the given object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            ob.drawOrder = order
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        obj.drawOrder = order
    else:
        error_log(fmt("Can't set draw order for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectGetDrawOrder*(name: string): int32 =
    ## Returns the draw order of the given object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            return ob.drawOrder

    let ob = gmFindObject(name)

    if ob.isSome():
        let obj = ob.get()
        return obj.drawOrder
    else:
        error_log(fmt("Can't get draw order for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSetDrawOrderGroup*(group: string, order: int32) =
    ## Sets the draw order for all the objects in the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.drawOrder = order

proc gmObjectSetActive*(name: string, active: bool = true) =
    ## Set active flag for given object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            ob.active = active
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        obj.active = active
    else:
        error_log(fmt("Can't set active flag for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectToggleActive*(name: string) =
    ## Toggle active flag for given object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            ob.active = not ob.active
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        obj.active = not obj.active
    else:
        error_log(fmt("Can't toggle active flag for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectGetActive*(name: string): bool =
    ## Returns the given object active flag. Raise an exception of the object was nmot found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            return ob.active

    let ob = gmFindObject(name)

    if ob.isSome():
        let obj = ob.get()
        return obj.active
    else:
        error_log(fmt("Can't get active flag from '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSetActiveGroup*(group: string, active: bool = true) =
    ## Set active flag for all objects that belong to the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.active = active

proc gmObjectToggleActiveGroup*(group: string) =
    ## Toggle active flag for all objects that belong to the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.active = not ob.active

proc gmObjectSetVisible*(name: string, visible: bool = true) =
    ## Set visible flag for given object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            ob.visible = visible
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        obj.visible = visible
    else:
        error_log(fmt("Can't set visible flag for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectToggleVisible*(name: string) =
    ## Toggle visible flag for given object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            ob.visible = not ob.visible
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        obj.visible = not obj.visible
    else:
        error_log(fmt("Can't toggle visible flag for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectGetVisible*(name: string): bool =
    ## Returns the visibility flag for the given object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            return ob.visible

    let ob = gmFindObject(name)

    if ob.isSome():
        let obj = ob.get()
        return obj.visible
    else:
        error_log(fmt("Can't get visible flag from '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSetVisibleGroup*(group: string, visible: bool = true) =
    ## Set visible flag for all objects that belong to the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.visible = visible

proc gmObjectToggleVisibleGroup*(group: string) =
    ## Toggle visible flag for all objects that belong to the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.visible = not ob.visible

proc gmObjectSendMessage*(name: string, message: JsonNode): JsonNode =
    ## Sends a message to the given object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            return ob.gmSendMessageIntern(message)

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        return obj.gmSendMessageIntern(message)
    else:
        error_log(fmt("Can't send message to '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSendMultiMessages*(name: string, messages: seq[JsonNode]): seq[JsonNode] =
    ## Sends multiple messages to the given object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            var r: seq[JsonNode] = @[]
#            for m in messages:
#                r.add(ob.gmSendMessageIntern(m))
#            return r

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        var r: seq[JsonNode] = @[]
        for m in messages:
            r.add(obj.gmSendMessageIntern(m))
        return r
    else:
        error_log(fmt("Can't send multiple message to '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSendMessageGroup*(group: string, message: JsonNode): seq[(string, JsonNode)] =
    ## Send a message to all objects that belong to the given group.
    result = @[]

    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            let v = ob.gmSendMessageIntern(message)
            result.add((ob.name, v))

proc gmObjectSendMultiMessagesGroup*(group: string, messages: seq[JsonNode]): seq[(string, seq[JsonNode])] =
    ## Sends multiple messages to all objects that belong to the given group.
    result = @[]

    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            var r: seq[JsonNode] = @[]
            for m in messages:
                r.add(ob.gmSendMessageIntern(m))
            result.add((ob.name, r))

proc gmObjectSetCustomProperty*(name: string, property: string, value: JsonNode) =
    ## Sets a custom property for the object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            ob.custom[property] = value
#            return

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        obj.custom[property] = value
    else:
        error_log(fmt("Can't set custom property for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectSetCustomPropertyGroup*(group: string, property: string, value: JsonNode) =
    ## Sets a custom property for all the objects in the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            ob.custom[property] = value

proc gmObjectGetCustomProperty*(name: string, property: string): JsonNode =
    ## Gets a custom property for the object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            if ob.custom.contains(property):
#                return ob.custom[property]
#            else:
#                return newJNull()

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        if obj.custom.contains(property):
            return obj.custom[property]
        else:
            return newJNull()
    else:
        error_log(fmt("Can't get custom property for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectRemoveCustomProperty*(name: string, property: string) =
    ## Removes a custom property for the object. Raise an exception if the object was not found.
#    for ob in GMGlobObjects.objects.mitems():
#        if ob.name == name:
#            if ob.custom.contains(property):
#                ob.custom.delete(property)

    let ob = gmFindObject(name)

    if ob.isSome():
        var obj = ob.get()
        if obj.custom.contains(property):
            obj.custom.delete(property)
    else:
        error_log(fmt("Can't remove custom property for '{name}', object not found!"), GMObjectNotFoundError)

proc gmObjectRemoveCustomPropertyGroup*(group: string, property: string) =
    ## Removes a custom property from all the objects that belong to the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            if ob.custom.contains(property):
                ob.custom.delete(property)

proc gmObjectApplyAll*(op: proc (o: var GMObject) {.closure.}) =
    ## Apply the given function 'op' to all objects.
    for ob in GMGlobObjects.objects.mitems():
        op(ob)

proc gmObjectApplyGroup*(group: string, op: proc (o: var GMObject) {.closure.}) =
    ## Apply the given function 'op' to all objects that belong to the given group.
    for ob in GMGlobObjects.objects.mitems():
        if group in ob.groups:
            op(ob)

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

