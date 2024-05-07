## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This file contains the test case for object management
##

# Nim std imports
import std/json
#import std/sets
import std/options

from std/strformat import fmt

# Local imports
import green_moon_2d/gm_object
import green_moon_2d/gm_json

type
    TestObject = ref object of GMObject
        drawCalled: uint8
        updateCalled: uint8
        messages: seq[JsonNode]

var testObjectOrder: seq[string]

method gmDraw*(self: var TestObject) =
    self.drawCalled += 1
    testObjectOrder.add(self.gmGetName())

method gmUpdate*(self: var TestObject) =
    self.updateCalled += 1
    testObjectOrder.add(self.gmGetName())

method gmSendMessage*(self: var TestObject, message: JsonNode): JsonNode =
    result = newJNull()

    self.messages.add(message)

    if message.contains("result"):
        result = message["result"]

#proc test_() =
#    gmInitSceneManager()
#    testObjectOrder = @[]

proc checkOneProperties(obj: TestObject, property: JsonNode) =
    let name = gmGetString(property, "name")
    let groups = gmGetHashSetString(property, "groups")
    let updateOrder = gmGetInt32(property, "updateOrder")
    let drawOrder = gmGetInt32(property, "drawOrder")
    let active = gmGetBool(property, "active")
    let visible = gmGetBool(property, "visible")
    let receiveBaseMessage = gmGetBool(property, "receiveBaseMessage")
    let x = gmGetFloat32(property, "x")
    let y = gmGetFloat32(property, "y")
    let custom = gmGetNode(property, "custom")
    let drawCalled = gmGetUint8(property, "drawCalled")
    let updateCalled = gmGetUint8(property, "updateCalled")
    let messages = gmGetNodes(property, "messages")

    assert(obj.gmGetName() == name)
    assert(obj.groups == groups)
    assert(obj.updateOrder == updateOrder)
    assert(obj.drawOrder == drawOrder)
    assert(obj.active == active)
    assert(obj.visible == visible)
    assert(obj.receiveBaseMessage == receiveBaseMessage)
    assert(obj.x == x)
    assert(obj.y == y)
    assert(obj.custom == custom)
    assert(obj.drawCalled == drawCalled)
    assert(obj.updateCalled == updateCalled)
    assert(obj.messages == messages)

proc test1_addObject1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    let ob1 = TestObject()

    gmAddObject("test1", ob1)

when isMainModule:
    test1_addObject1()

