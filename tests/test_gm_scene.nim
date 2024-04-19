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
    var scene1 = TestScene()
    gmAddScene(scene1, "Test1")
    gmEnterScene("Test1")
    checkOneProperty(scene1, "enterCalled", 1u8)

when isMainModule:
    gmInitSceneManager()
    test1_addScene1()

