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

proc checkProperties1(obj: GMObject, property: JsonNode) =
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

proc checkProperties1Name(name: string, property: JsonNode) =
    let ob = gmFindObject(name)
    assert(ob.isSome())
    let ob2 = ob.get()
    checkProperties1(ob2, property)

proc checkProperties2(obj: TestObject, property: JsonNode) =
    checkProperties1(obj, property)

    let drawCalled = gmGetUint8(property, "drawCalled")
    let updateCalled = gmGetUint8(property, "updateCalled")
    let messages = gmGetNodes(property, "messages")

    assert2(obj.drawCalled, drawCalled, "drawCalled")
    assert2(obj.updateCalled, updateCalled, "updateCalled")
    assert2(obj.messages, messages, "messages")

proc test1_addObject1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    assert(gmGetNumberOfObjects() == 0)

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test1", GMObject(ob1))
    checkProperties2(ob1, %*{"name": "test1"})
    assert(gmGetNumberOfObjects() == 1)

    gmAddObject("test2", GMObject(ob2))
    checkProperties2(ob2, %*{"name": "test2"})
    assert(gmGetNumberOfObjects() == 2)

    discard gmObjectSendMessage("test1", %*{"setX": 12.0})
    checkProperties2(ob1, %*{"name": "test1", "x": 12.0})

    discard gmObjectSendMessage("test2", %*{"setY": -99.0})
    checkProperties2(ob2, %*{"name": "test2", "y": -99.0})

proc test2_addObject2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test2", GMObject(ob1))
    checkProperties2(ob1, %*{"name": "test2"})

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
    checkProperties2(ob1, %*{"name": "test3", "x": 19.0})

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
    assert(gmGetNumberOfObjects() == 2)

    gmDeleteObject("test5.1")
    assert(gmGetNumberOfObjects() == 1)

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
    checkProperties2(ob1, %*{"name": "test7.1", "groups": ["poison"]})
    checkProperties2(ob2, %*{"name": "test7.2"})

    gmObjectAddGroup("test7.1", "strong")
    checkProperties2(ob1, %*{"name": "test7.1", "groups": ["poison", "strong"]})
    checkProperties2(ob2, %*{"name": "test7.2"})

    gmObjectAddGroup("test7.2", "unlucky")
    checkProperties2(ob1, %*{"name": "test7.1", "groups": ["poison", "strong"]})
    checkProperties2(ob2, %*{"name": "test7.2", "groups": ["unlucky"]})

    gmObjectAddGroup("test7.2", "unlucky")
    checkProperties2(ob1, %*{"name": "test7.1", "groups": ["poison", "strong"]})
    checkProperties2(ob2, %*{"name": "test7.2", "groups": ["unlucky"]})

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
    checkProperties2(ob1, %*{"name": "test9.1", "groups": ["bullet", "magnetic"]})
    checkProperties2(ob2, %*{"name": "test9.2"})

    gmObjectAddGroups("test9.2", ["bleed", "fast", "infected"])
    checkProperties2(ob1, %*{"name": "test9.1", "groups": ["bullet", "magnetic"]})
    checkProperties2(ob2, %*{"name": "test9.2", "groups": ["bleed", "fast", "infected"]})

    gmObjectAddGroups("test9.2", [])
    checkProperties2(ob1, %*{"name": "test9.1", "groups": ["bullet", "magnetic"]})
    checkProperties2(ob2, %*{"name": "test9.2", "groups": ["bleed", "fast", "infected"]})

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
    checkProperties2(ob1, %*{"name": "test11.1", "groups": ["small"]})
    checkProperties2(ob2, %*{"name": "test11.2"})

    gmObjectRemoveGroup("test11.1", "small")
    checkProperties2(ob1, %*{"name": "test11.1"})
    checkProperties2(ob2, %*{"name": "test11.2"})

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
    checkProperties2(ob1, %*{"name": "test13.1", "groups": ["blue"]})
    checkProperties2(ob2, %*{"name": "test13.2"})

    gmObjectRemoveGroups("test13.1", [])
    checkProperties2(ob1, %*{"name": "test13.1", "groups": ["blue"]})
    checkProperties2(ob2, %*{"name": "test13.2"})

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

    checkProperties2(ob1, %*{"name": "test17.1", "groups": ["green", "red"]})
    checkProperties2(ob2, %*{"name": "test17.2", "groups": ["yellow"]})
    checkProperties2(ob3, %*{"name": "test17.3"})
    checkProperties2(ob4, %*{"name": "test17.4", "groups": ["pink", "yellow"]})

proc test18_deleteObjectsInGroup1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()
    var ob3 = TestObject()
    var ob4 = TestObject()
    var ob5 = TestObject()

    gmAddObject("test18.1", GMObject(ob1))
    gmAddObject("test18.2", GMObject(ob2))
    gmAddObject("test18.3", GMObject(ob3))
    gmAddObject("test18.4", GMObject(ob4))
    gmAddObject("test18.5", GMObject(ob5))
    assert(gmGetNumberOfObjects() == 5)

    gmObjectAddGroups("test18.1", ["green"])
    gmObjectAddGroups("test18.2", ["green", "blue"])
    gmObjectAddGroups("test18.3", ["red", "green"])
    gmObjectAddGroups("test18.4", ["red", "blue"])

    gmDeleteObjectsInGroup("green")
    assert(gmGetNumberOfObjects() == 2)

    let ob6 = gmFindObject("test18.1")
    assert(ob6.isNone())

    let ob7 = gmFindObject("test18.2")
    assert(ob7.isNone())

    let ob8 = gmFindObject("test18.3")
    assert(ob8.isNone())

    checkProperties1Name("test18.4", %*{"name": "test18.4", "groups": ["red", "blue"]})
    checkProperties1Name("test18.5", %*{"name": "test18.5"})

proc test19_deleteObjectsInGroup2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()
    var ob3 = TestObject()
    var ob4 = TestObject()
    var ob5 = TestObject()

    gmAddObject("test19.1", GMObject(ob1))
    gmAddObject("test19.2", GMObject(ob2))
    gmAddObject("test19.3", GMObject(ob3))
    gmAddObject("test19.4", GMObject(ob4))
    gmAddObject("test19.5", GMObject(ob5))
    assert(gmGetNumberOfObjects() == 5)

    gmObjectAddGroups("test19.1", ["green"])
    gmObjectAddGroups("test19.2", ["blue"])
    gmObjectAddGroups("test19.3", ["red", "green"])
    gmObjectAddGroups("test19.4", ["red", "blue"])
    gmObjectAddGroups("test19.5", ["green", "yellow"])

    gmDeleteObjectsInGroup("green")
    assert(gmGetNumberOfObjects() == 2)

    let ob6 = gmFindObject("test19.1")
    assert(ob6.isNone())

    let ob7 = gmFindObject("test19.3")
    assert(ob7.isNone())

    let ob8 = gmFindObject("test19.5")
    assert(ob8.isNone())

    checkProperties1Name("test19.2", %*{"name": "test19.2", "groups": ["blue"]})
    checkProperties1Name("test19.4", %*{"name": "test19.4", "groups": ["red", "blue"]})

proc test20_deleteObjectsInGroup3() =
    gmInitObjectManager()
    testObjectOrder = @[]

    # Try to remove objects if there are none available:
    gmDeleteObjectsInGroup("green")
    assert(gmGetNumberOfObjects() == 0)

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test20.1", GMObject(ob1))
    gmAddObject("test20.2", GMObject(ob2))
    assert(gmGetNumberOfObjects() == 2)

    gmObjectAddGroups("test20.1", ["red"])
    gmObjectAddGroups("test20.2", ["blue"])

    # This group does not exist:
    gmDeleteObjectsInGroup("green")
    assert(gmGetNumberOfObjects() == 2)

    checkProperties1Name("test20.1", %*{"name": "test20.1", "groups": ["red"]})
    checkProperties1Name("test20.2", %*{"name": "test20.2", "groups": ["blue"]})

proc test21_setUpdateOrder1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test21.1", GMObject(ob1))
    gmAddObject("test21.2", GMObject(ob2))

    gmObjectSetUpdateOrder("test21.1", 5)

    checkProperties2(ob1, %*{"name": "test21.1", "updateOrder": 5})
    checkProperties2(ob2, %*{"name": "test21.2", "updateOrder": 0})

    gmObjectSetUpdateOrder("test21.2", -11)

    checkProperties2(ob1, %*{"name": "test21.1", "updateOrder": 5})
    checkProperties2(ob2, %*{"name": "test21.2", "updateOrder": -11})

proc test22_setUpdateOrder2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test22.1", GMObject(ob1))

    doAssertRaises GMObjectNotFoundError:
        gmObjectSetUpdateOrder("test22.2", 11)

proc test23_getUpdateOrder1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test23.1", GMObject(ob1))

    gmObjectSetUpdateOrder("test23.1", 7)

    assert(gmObjectGetUpdateOrder("test23.1") == 7)

proc test24_getUpdateOrder2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test24.1", GMObject(ob1))

    gmObjectSetUpdateOrder("test24.1", 7)

    doAssertRaises GMObjectNotFoundError:
        discard gmObjectGetUpdateOrder("test24.2")

proc test25_setUpdateOrderGroup1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()
    var ob3 = TestObject()
    var ob4 = TestObject()
    var ob5 = TestObject()

    gmAddObject("test25.1", GMObject(ob1))
    gmAddObject("test25.2", GMObject(ob2))
    gmAddObject("test25.3", GMObject(ob3))
    gmAddObject("test25.4", GMObject(ob4))
    gmAddObject("test25.5", GMObject(ob5))

    gmObjectAddGroups("test25.1", ["green"])
    gmObjectAddGroups("test25.2", ["blue", "red"])
    gmObjectAddGroups("test25.3", ["yellow", "blue"])
    gmObjectAddGroups("test25.4", ["gray"])

    gmObjectSetUpdateOrderGroup("blue", 37)

    checkProperties2(ob1, %*{"name": "test25.1", "groups": ["green"]})
    checkProperties2(ob2, %*{"name": "test25.2", "groups": ["blue", "red"], "updateOrder": 37})
    checkProperties2(ob3, %*{"name": "test25.3", "groups": ["yellow", "blue"],  "updateOrder": 37})
    checkProperties2(ob4, %*{"name": "test25.4", "groups": ["gray"]})
    checkProperties2(ob5, %*{"name": "test25.5"})

proc test26_setDrawOrder1() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    var ob2 = TestObject()

    gmAddObject("test26.1", GMObject(ob1))
    gmAddObject("test26.2", GMObject(ob2))

    gmObjectSetDrawOrder("test26.1", 5)

    checkProperties2(ob1, %*{"name": "test26.1", "drawOrder": 5})
    checkProperties2(ob2, %*{"name": "test26.2", "drawOrder": 0})

    gmObjectSetDrawOrder("test26.2", -11)

    checkProperties2(ob1, %*{"name": "test26.1", "drawOrder": 5})
    checkProperties2(ob2, %*{"name": "test26.2", "drawOrder": -11})

proc test27_setDrawOrder2() =
    gmInitObjectManager()
    testObjectOrder = @[]

    var ob1 = TestObject()
    gmAddObject("test27.1", GMObject(ob1))

    doAssertRaises GMObjectNotFoundError:
        gmObjectSetDrawOrder("test27.2", 11)


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
    test18_deleteObjectsInGroup1()
    test19_deleteObjectsInGroup2()
    test20_deleteObjectsInGroup3()
    test21_setUpdateOrder1()
    test22_setUpdateOrder2()
    test23_getUpdateOrder1()
    test24_getUpdateOrder2()
    test25_setUpdateOrderGroup1()
    test26_setDrawOrder1()
    test27_setDrawOrder2()
    #test28_getDrawOrder1()
    #test29_getDrawOrder2()
    #test30_setDrawOrderGroup1()
    #test31_setActive1()
    #test32_setActive2()





