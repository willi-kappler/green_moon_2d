## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This file contains the test case for scene management
##

# Nim std imports
import std/json
#from std/files import removeFile
#from std/paths import Path
#from std/strformat import fmt

# Local imports
import green_moon_2d/gm_scene

type
    TestScene = ref object of GMScene
        enterCalled: uint8
        drawCalled: uint8
        updateCalled: uint8
        message1Sent: uint8
        message2Sent: uint8

method gmEnter*(self: var TestScene) =
    inc(self.enterCalled)

method gmDraw*(self: var TestScene) =
    inc(self.drawCalled)

method gmUpdate*(self: var TestScene) =
    inc(self.updateCalled)

method gmCustom*(self: var TestScene, data: JsonNode): JsonNode =
    if data.contains("message1"):
        inc(self.message1Sent)
    elif data.contains("message2"):
        inc(self.message2Sent)

proc checkOneProperty(scene: TestScene, name: string = "", value: uint8 = 0) =
    if name == "enterCalled":
        assert(scene.enterCalled == value)
    else:
        assert(scene.enterCalled == 0)

    if name == "drawCalled":
        assert(scene.drawCalled == value)
    else:
        assert(scene.drawCalled == 0)

    if name == "updateCalled":
        assert(scene.updateCalled == value)
    else:
        assert(scene.updateCalled == 0)

    if name == "message1Sent":
        assert(scene.message1Sent == value)
    else:
        assert(scene.message1Sent == 0)

    if name == "message2Sent":
        assert(scene.message2Sent == value)
    else:
        assert(scene.message2Sent == 0)

proc test1_addScene1() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test1")
    gmEnterScene("Test1")
    checkOneProperty(scene1, "enterCalled", 1u8)

proc test2_addScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test2.1")
    gmAddScene(scene2, "Test2.2")

    gmEnterScene("Test2.1")
    gmUpdateScene("Test2.2")

    checkOneProperty(scene1, "enterCalled", 1u8)
    checkOneProperty(scene2, "updateCalled", 1u8)

proc test3_addScene3() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test3")

    doAssertRaises GMSceneAlreadyExistsError:
        gmAddScene(scene1, "Test3")

proc test4_drawScene1() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test4.1")
    gmAddScene(scene2, "Test4.2")

    gmDrawScene("Test4.1")
    checkOneProperty(scene1, "drawCalled", 1u8)
    checkOneProperty(scene2)

    gmDrawScene("Test4.2")
    gmDrawScene("Test4.2")
    checkOneProperty(scene1, "drawCalled", 1u8)
    checkOneProperty(scene2, "drawCalled", 2u8)

proc test5_drawScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test5.1")
    gmAddScene(scene2, "Test5.2")

    gmDrawCurrentScene()

    checkOneProperty(scene1, "drawCalled", 1u8)
    checkOneProperty(scene2)

proc test6_drawScene3() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test6.1")

    doAssertRaises GMSceneNotFoundError:
        gmDrawScene("Test1.1")

proc test7_updateScene1() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test7.1")
    gmAddScene(scene2, "Test7.2")

    gmUpdateScene("Test7.1")
    checkOneProperty(scene1, "updateCalled", 1u8)
    checkOneProperty(scene2)

    gmUpdateScene("Test7.2")
    gmUpdateScene("Test7.2")
    checkOneProperty(scene1, "updateCalled", 1u8)
    checkOneProperty(scene2, "updateCalled", 2u8)

proc test8_updateScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test8.1")
    gmAddScene(scene2, "Test8.2")

    gmUpdateCurrentScene()

    checkOneProperty(scene1, "updateCalled", 1u8)
    checkOneProperty(scene2)

proc test9_updateScene3() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test9.1")

    doAssertRaises GMSceneNotFoundError:
        gmUpdateScene("Test1.1")

proc test10_enterScene1() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test10.1")
    gmAddScene(scene2, "Test10.2")

    gmEnterScene("Test10.1")

    checkOneProperty(scene1, "enterCalled", 1u8)
    checkOneProperty(scene2)

    gmEnterScene("Test10.2")
    gmEnterScene("Test10.2")

    checkOneProperty(scene1, "enterCalled", 1u8)
    checkOneProperty(scene2, "enterCalled", 2u8)

proc test11_enterScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test11.1")

    doAssertRaises GMSceneNotFoundError:
        gmEnterScene("Test11.2")

proc test12_removeScene1() =
    gmInitSceneManager()

    # Can't remove the current active scene, so we need a second one
    # to be removed.
    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test12.1")
    gmAddScene(scene2, "Test12.2")
    gmEnterScene("Test12.2")
    checkOneProperty(scene1)
    checkOneProperty(scene2, "enterCalled", 1u8)

    gmRemoveScene("Test12.2")
    doAssertRaises GMSceneNotFoundError:
        gmEnterScene("Test12.2")

proc test13_removeScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test13.1")

    doAssertRaises GMSceneIsActiveError:
        gmRemoveScene("Test13.1")

proc test14_removeScene3() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test14.1")

    doAssertRaises GMSceneNotFoundError:
        gmRemoveScene("Test14.2")

proc test15_removeScene4() =
    gmInitSceneManager()

    doAssertRaises GMSceneNotFoundError:
        gmRemoveScene("Test15.1")

proc test16_changeScene1() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test16.1")
    gmAddScene(scene2, "Test16.2")

    gmDrawCurrentScene()

    checkOneProperty(scene1, "drawCalled", 1u8)
    checkOneProperty(scene2)

    # This also calls enter scene!
    gmChangeScene("Test16.2")

    gmUpdateCurrentScene()

    checkOneProperty(scene1, "drawCalled", 1u8)
    assert(scene2.enterCalled == 1)
    assert(scene2.drawCalled == 0)
    assert(scene2.updateCalled == 1)
    assert(scene2.message1Sent == 0)
    assert(scene2.message2Sent == 0)

proc test17_changeScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test17.1")

    doAssertRaises GMSceneIsActiveError:
        gmChangeScene("Test17.1")

proc test18_changeScene3() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test18.1")

    doAssertRaises GMSceneNotFoundError:
        gmChangeScene("Test18.2")

proc test19_replaceScene1() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test19.1")

    gmUpdateCurrentScene()

    checkOneProperty(scene1, "updateCalled", 1u8)
    checkOneProperty(scene2)

    gmReplaceScene("Test19.1", scene2)

    checkOneProperty(scene1, "updateCalled", 1u8)
    checkOneProperty(scene2)

    gmDrawCurrentScene()

    checkOneProperty(scene1, "updateCalled", 1u8)
    checkOneProperty(scene2, "drawCalled", 1u8)

proc test20_replaceScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test20.1")

    doAssertRaises GMSceneNotFoundError:
        gmReplaceScene("Test20.2", scene2)



when isMainModule:
    test1_addScene1()
    test2_addScene2()
    test3_addScene3()
    test4_drawScene1()
    test5_drawScene2()
    test6_drawScene3()
    test7_updateScene1()
    test8_updateScene2()
    test9_updateScene3()
    test10_enterScene1()
    test11_enterScene2()
    test12_removeScene1()
    test13_removeScene2()
    test14_removeScene3()
    test15_removeScene4()
    test16_changeScene1()
    test17_changeScene2()
    test18_changeScene3()
    test19_replaceScene1()
    test20_replaceScene2()
    # push and change scene
    # push and change scene active
    # push and change scene error not found
    # pop and change scene
    # pop and change scene error empty stack
    # custom message
    # custom message scene error not found





