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
import std/sets
import std/options

from std/strformat import fmt

# Local imports
import green_moon_2d/gm_object

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
    var name = ""
    var groups: HashSet[string] = initHashSet[string]()
    var updateOrder: int32 = 0
    var drawOrder: int32 = 0
    var active = false
    var visible = false
    var receiveBaseMessage = false
    var x: float32 = 0.0
    var y: float32 = 0.0
    var custom = newJNull()
    var drawCalled: uint8 = 0
    var updateCalled: uint8 = 0
    var messages: seq[JsonNode] = @[]


    assert(obj.groups == groups)


proc test1_addObject1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    let ob1 = TestObject()

    gmAddObject("test1", ob1)

when isMainModule:
    test1_addObject1()

