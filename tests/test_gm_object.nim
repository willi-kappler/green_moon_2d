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
import green_moon_2d/gm_util

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

proc checkProperties(obj: TestObject, property: JsonNode) =
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

    assert2(obj.gmGetName(), name, "name")
    assert2(obj.groups, groups, "groups")
    assert2(obj.updateOrder, updateOrder, "updateOrder")
    assert2(obj.drawOrder, drawOrder, "drawOrder")
    assert2(obj.active, active, "active")
    assert2(obj.visible, visible, "visible")
    assert2(obj.receiveBaseMessage, receiveBaseMessage, "receiveBaseMessage")
    assert2(obj.x, x, "x")
    assert2(obj.y, y, "y")
    assert2(obj.custom, custom, "custom")
    assert2(obj.drawCalled, drawCalled, "drawCalled")
    assert2(obj.updateCalled, updateCalled, "updateCalled")
    assert2(obj.messages, messages, "messages")

proc test1_addObject1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    let ob1 = TestObject()

    gmAddObject("test1", ob1)

when isMainModule:
    test1_addObject1()

