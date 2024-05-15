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
    assert2(obj.position.x, x, "x")
    assert2(obj.position.y, y, "y")
    assert2(obj.custom, custom, "custom")
    assert2(obj.drawCalled, drawCalled, "drawCalled")
    assert2(obj.updateCalled, updateCalled, "updateCalled")
    assert2(obj.messages, messages, "messages")

proc test1_addObject1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test1", GMObject(ob1))
    checkProperties(ob1, %*{"name": "test1"})

    gmAddObject("test2", GMObject(ob2))
    checkProperties(ob2, %*{"name": "test2"})

    discard gmObjectSendMessage("test1", %*{"setX": 12.0})
    checkProperties(ob1, %*{"name": "test1", "x": 12.0})

    discard gmObjectSendMessage("test2", %*{"setY": -99.0})
    checkProperties(ob2, %*{"name": "test2", "y": -99.0})

proc test2_addObject2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test2", GMObject(ob1))
    checkProperties(ob1, %*{"name": "test2"})

    doAssertRaises GMObjectAlreadyExistsError:
        gmAddObject("test2", GMObject(ob2))

proc test3_findObject1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test3", GMObject(ob1))
    discard gmObjectSendMessage("test3", %*{"setX": 19.0})
    let ob2 = gmFindObject("test3")

    assert(ob2.isSome())
    let ob2_ok = ob2.get()
    assert(ob2_ok.gmGetName() == "test3")
    assert(ob2_ok.position.x == 19.0)
    checkProperties(ob1, %*{"name": "test3", "x": 19.0})

proc test4_findObject2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test4.1", GMObject(ob1))
    let ob2 = gmFindObject("test4.2")

    assert(ob2.isNone())

proc test5_removeObject1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()
    gmAddObject("test5.1", GMObject(ob1))
    gmAddObject("test5.2", GMObject(ob2))

    gmDeleteObject("test5.1")

    assert(gmFindObject("test5.1").isNone())
    assert(gmFindObject("test5.2").isSome())

proc test6_removeObject2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test6.1", GMObject(ob1))

    doAssertRaises GMObjectNotFoundError:
        gmDeleteObject("test6.2")

proc test7_addGroup1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test7.1", GMObject(ob1))
    gmAddObject("test7.2", GMObject(ob2))

    gmObjectAddGroup("test7.1", "poison")
    checkProperties(ob1, %*{"name": "test7.1", "groups": ["poison"]})
    checkProperties(ob2, %*{"name": "test7.2"})

    gmObjectAddGroup("test7.1", "strong")
    checkProperties(ob1, %*{"name": "test7.1", "groups": ["poison", "strong"]})
    checkProperties(ob2, %*{"name": "test7.2"})

    gmObjectAddGroup("test7.2", "unlucky")
    checkProperties(ob1, %*{"name": "test7.1", "groups": ["poison", "strong"]})
    checkProperties(ob2, %*{"name": "test7.2", "groups": ["unlucky"]})

    gmObjectAddGroup("test7.2", "unlucky")
    checkProperties(ob1, %*{"name": "test7.1", "groups": ["poison", "strong"]})
    checkProperties(ob2, %*{"name": "test7.2", "groups": ["unlucky"]})

proc test8_addGroup2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test8.1", GMObject(ob1))

    doAssertRaises GMObjectNotFoundError:
        gmObjectAddGroup("test8.2", "poison")

proc test9_addGroups1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test9.1", GMObject(ob1))
    gmAddObject("test9.2", GMObject(ob2))

    gmObjectAddGroups("test9.1", ["bullet", "magnetic"])
    checkProperties(ob1, %*{"name": "test9.1", "groups": ["bullet", "magnetic"]})
    checkProperties(ob2, %*{"name": "test9.2"})

    gmObjectAddGroups("test9.2", ["bleed", "fast", "infected"])
    checkProperties(ob1, %*{"name": "test9.1", "groups": ["bullet", "magnetic"]})
    checkProperties(ob2, %*{"name": "test9.2", "groups": ["bleed", "fast", "infected"]})

    gmObjectAddGroups("test9.2", [])
    checkProperties(ob1, %*{"name": "test9.1", "groups": ["bullet", "magnetic"]})
    checkProperties(ob2, %*{"name": "test9.2", "groups": ["bleed", "fast", "infected"]})

proc test10_addGroups2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test10.1", GMObject(ob1))

    doAssertRaises GMObjectNotFoundError:
        gmObjectAddGroups("test10.2", ["poison", "unholy"])

proc test11_removeGroup1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test11.1", GMObject(ob1))
    gmAddObject("test11.2", GMObject(ob2))

    gmObjectAddGroups("test11.1", ["small", "wounded"])
    gmObjectRemoveGroup("test11.1", "wounded")
    checkProperties(ob1, %*{"name": "test11.1", "groups": ["small"]})
    checkProperties(ob2, %*{"name": "test11.2"})

    gmObjectRemoveGroup("test11.1", "small")
    checkProperties(ob1, %*{"name": "test11.1"})
    checkProperties(ob2, %*{"name": "test11.2"})

proc test12_removeGroup2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test12.1", GMObject(ob1))

    doAssertRaises GMObjectNotFoundError:
        gmObjectRemoveGroup("test12.2", "foo")

proc test13_removeGroups1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test13.1", GMObject(ob1))
    gmAddObject("test13.2", GMObject(ob2))

    gmObjectAddGroups("test13.1", ["green", "red", "blue"])

    gmObjectRemoveGroups("test13.1", ["green", "red"])
    checkProperties(ob1, %*{"name": "test13.1", "groups": ["blue"]})
    checkProperties(ob2, %*{"name": "test13.2"})

    gmObjectRemoveGroups("test13.1", [])
    checkProperties(ob1, %*{"name": "test13.1", "groups": ["blue"]})
    checkProperties(ob2, %*{"name": "test13.2"})

proc test14_removeGroups2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test14.1", GMObject(ob1))

    doAssertRaises GMObjectNotFoundError:
        gmObjectRemoveGroups("test14.2", ["foo"])

proc test15_objectInGroup1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test15.1", GMObject(ob1))
    gmAddObject("test15.2", GMObject(ob2))

    gmObjectAddGroups("test15.1", ["green", "red", "blue"])
    assert(gmObjectInGroup("test15.1", "green"))
    assert(gmObjectInGroup("test15.1", "red"))
    assert(gmObjectInGroup("test15.1", "blue"))
    assert(not gmObjectInGroup("test15.1", "yellow"))
    assert(not gmObjectInGroup("test15.2", "green"))

proc test16_objectInGroup2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test16.1", GMObject(ob1))

    doAssertRaises GMObjectNotFoundError:
        discard gmObjectInGroup("test16.2", "foo")

proc test17_objectRemoveGroupFromAll1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()
    var ob3 = TestObject()
    var ob4 = TestObject()

    gmAddObject("test17.1", GMObject(ob1))
    gmAddObject("test17.2", GMObject(ob2))
    gmAddObject("test17.3", GMObject(ob3))
    gmAddObject("test17.4", GMObject(ob4))

    gmObjectAddGroups("test17.1", ["green", "red", "blue"])
    gmObjectAddGroups("test17.2", ["yellow", "blue"])
    gmObjectAddGroups("test17.3", ["blue"])
    gmObjectAddGroups("test17.4", ["pink", "yellow"])

    gmObjectRemoveGroupFromAll("blue")

    checkProperties(ob1, %*{"name": "test17.1", "groups": ["green", "red"]})
    checkProperties(ob2, %*{"name": "test17.2", "groups": ["yellow"]})
    checkProperties(ob3, %*{"name": "test17.3"})
    checkProperties(ob4, %*{"name": "test17.4", "groups": ["pink", "yellow"]})

when isMainModule:
    test1_addObject1()
    test2_addObject2()
    test3_findObject1()
    test4_findObject2()
    test5_removeObject1()
    test6_removeObject2()
    test7_addGroup1()
    test8_addGroup2()
    test9_addGroups1()
    test10_addGroups2()
    test11_removeGroup1()
    test12_removeGroup2()
    test13_removeGroups1()
    test14_removeGroups2()
    test15_objectInGroup1()
    test16_objectInGroup2()
    test17_objectRemoveGroupFromAll1()



