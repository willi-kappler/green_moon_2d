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
from std/strformat import fmt

# Local imports
import green_moon_2d/gm_scene
import green_moon_2d/gm_json
import green_moon_2d/gm_util

type
    TestScene = ref object of GMScene
        enterCalled: uint8
        drawCalled: uint8
        drawAfterCalled: uint8
        updateCalled: uint8
        updateAfterCalled: uint8
        message1Sent: uint8
        message2Sent: uint8

method gmEnter*(self: var TestScene) =
    inc(self.enterCalled)

method gmDraw*(self: var TestScene) =
    inc(self.drawCalled)

method gmDrawAfter*(self: var TestScene) =
    inc(self.drawAfterCalled)

method gmUpdate*(self: var TestScene) =
    inc(self.updateCalled)

method gmUpdateAfter*(self: var TestScene) =
    inc(self.updateAfterCalled)

method gmSendMessage*(self: var TestScene, data: JsonNode): JsonNode =
    if data.contains("message1"):
        inc(self.message1Sent)
    if data.contains("message2"):
        inc(self.message2Sent)

proc checkProperties(scene: TestScene, node: JsonNode = newJNull()) =
    let enterCalled = gmGetUint8(node, "enterCalled")
    let drawCalled = gmGetUint8(node, "drawCalled")
    let drawAfterCalled = gmGetUint8(node, "drawAfterCalled")
    let updateCalled = gmGetUint8(node, "updateCalled")
    let updateAfterCalled = gmGetUint8(node, "updateAfterCalled")
    let message1Sent = gmGetUint8(node, "message1Sent")
    let message2Sent = gmGetUint8(node, "message2Sent")

    assert2(scene.enterCalled, enterCalled, "enterCalled")
    assert2(scene.drawCalled, drawCalled, "drawCalled")
    assert2(scene.drawAfterCalled, drawAfterCalled, "drawAfterCalled")
    assert2(scene.updateCalled, updateCalled, "updateCalled")
    assert2(scene.updateAfterCalled, updateAfterCalled, "updateAfterCalled")
    assert2(scene.message1Sent, message1Sent, "message1Sent")
    assert2(scene.message2Sent, message2Sent, "message2Sent")

proc test1_addScene1() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test1")
    gmEnterScene("Test1")
    checkProperties(scene1, %*{"enterCalled": 1})

proc test2_addScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test2.1")
    gmAddScene(scene2, "Test2.2")

    gmEnterScene("Test2.1")
    gmUpdateScene("Test2.2")
    checkProperties(scene1, %*{"enterCalled": 1})
    checkProperties(scene2, %*{"updateCalled": 1})

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
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2)

    gmDrawScene("Test4.2")
    gmDrawScene("Test4.2")
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2, %*{"drawCalled": 2})

proc test5_drawScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test5.1")
    gmAddScene(scene2, "Test5.2")

    gmDrawCurrentScene()
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2)

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
    checkProperties(scene1, %*{"updateCalled": 1})
    checkProperties(scene2)

    gmUpdateScene("Test7.2")
    gmUpdateScene("Test7.2")
    checkProperties(scene1, %*{"updateCalled": 1})
    checkProperties(scene2, %*{"updateCalled": 2})

proc test8_updateScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test8.1")
    gmAddScene(scene2, "Test8.2")

    gmUpdateCurrentScene()
    checkProperties(scene1, %*{"updateCalled": 1})
    checkProperties(scene2)

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
    checkProperties(scene1, %*{"enterCalled": 1})
    checkProperties(scene2)

    gmEnterScene("Test10.2")
    gmEnterScene("Test10.2")
    checkProperties(scene1, %*{"enterCalled": 1})
    checkProperties(scene2, %*{"enterCalled": 2})

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
    checkProperties(scene1)
    checkProperties(scene2, %*{"enterCalled": 1})

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
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2)

    # This also calls enter scene!
    gmChangeScene("Test16.2")

    gmUpdateCurrentScene()
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2, %*{"enterCalled": 1, "updateCalled": 1})

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
    checkProperties(scene1, %*{"updateCalled": 1})
    checkProperties(scene2)

    gmReplaceScene("Test19.1", scene2)
    checkProperties(scene1, %*{"updateCalled": 1})
    checkProperties(scene2)

    gmDrawCurrentScene()
    checkProperties(scene1, %*{"updateCalled": 1})
    checkProperties(scene2, %*{"drawCalled": 1})

proc test20_replaceScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test20.1")

    doAssertRaises GMSceneNotFoundError:
        gmReplaceScene("Test20.2", scene2)

proc test21_pushAndChangeScene1() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test21.1")
    gmAddScene(scene2, "Test21.2")
    checkProperties(scene1)
    checkProperties(scene2)

    gmDrawCurrentScene()
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2)

    # This calls enter scene!
    gmPushAndChangeScene("Test21.2")
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2, %*{"enterCalled": 1})

    gmUpdateCurrentScene()
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2, %*{"enterCalled": 1, "updateCalled": 1})

proc test22_pushAndChangeScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test22.1")

    doAssertRaises GMSceneIsActiveError:
        gmPushAndChangeScene("Test22.1")

proc test23_pushAndChangeScene3() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test23.1")

    doAssertRaises GMSceneNotFoundError:
        gmPushAndChangeScene("Test23.2")

proc test24_popAndChangeScene1() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test24.1")
    gmAddScene(scene2, "Test24.2")
    checkProperties(scene1)
    checkProperties(scene2)

    gmDrawCurrentScene()
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2)

    # This calls enter scene!
    gmPushAndChangeScene("Test24.2")
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2, %*{"enterCalled": 1})

    gmUpdateCurrentScene()
    checkProperties(scene1, %*{"drawCalled": 1})
    checkProperties(scene2, %*{"enterCalled": 1, "updateCalled": 1})

    gmPopAndChangeScene()
    checkProperties(scene1, %*{"enterCalled": 1, "drawCalled": 1})
    checkProperties(scene2, %*{"enterCalled": 1, "updateCalled": 1})

    gmUpdateCurrentScene()
    checkProperties(scene1, %*{"enterCalled": 1, "drawCalled": 1, "updateCalled": 1})
    checkProperties(scene2, %*{"enterCalled": 1, "updateCalled": 1})

proc test25_popAndChangeScene2() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test25.1")

    doAssertRaises GMSceneStackEmptyError:
        gmPopAndChangeScene()

proc test26_customMessage1() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test26.1")
    gmAddScene(scene2, "Test26.2")
    checkProperties(scene1)
    checkProperties(scene2)

    discard gmSceneSendMessage("Test26.1", %*{"message1": 12})
    checkProperties(scene1, %*{"message1Sent": 1})
    checkProperties(scene2)

    discard gmSceneSendMessage("Test26.2", %*{"message2": 15})
    checkProperties(scene1, %*{"message1Sent": 1})
    checkProperties(scene2, %*{"message2Sent": 1})

    discard gmSceneSendMessage("Test26.2", %*{"message1": 32, "message2": 15})
    checkProperties(scene1, %*{"message1Sent": 1})
    checkProperties(scene2, %*{"message1Sent": 1, "message2Sent": 2})

proc test27_customMessage2() =
    gmInitSceneManager()

    let scene1 = TestScene()

    gmAddScene(scene1, "Test27.1")

    doAssertRaises GMSceneNotFoundError:
        discard gmSceneSendMessage("Test27.2", %*{"message1": 12})

proc test28_drawSceneAfter() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test28.1")
    gmAddScene(scene2, "Test28.2")

    gmDrawCurrentSceneAfter()
    checkProperties(scene1, %*{"drawAfterCalled": 1})
    checkProperties(scene2)

proc test29_updateSceneAfter() =
    gmInitSceneManager()

    let scene1 = TestScene()
    let scene2 = TestScene()

    gmAddScene(scene1, "Test29.1")
    gmAddScene(scene2, "Test29.2")

    gmUpdateCurrentSceneAfter()
    checkProperties(scene1, %*{"updateAfterCalled": 1})
    checkProperties(scene2)

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
    test21_pushAndChangeScene1()
    test22_pushAndChangeScene2()
    test23_pushAndChangeScene3()
    test24_popAndChangeScene1()
    test25_popAndChangeScene2()
    test26_customMessage1()
    test27_customMessage2()
    test28_drawSceneAfter()
    test29_updateSceneAfter()

