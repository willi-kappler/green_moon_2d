## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This module contains the code that handles all the scenes.
##

# Nim std imports
import std/json

type
    GMScene* = ref object of RootObj
        name: string

    GMSceneManager* = object
        scenes: seq[GMScene]
        sceneStack: seq[string]
        currentScene: uint32

# GMScene:

method gmInit*(self: var GMScene) {.base.} =
    ## This method can be implemented when the scene is initialized.
    discard

method gmEnter*(self: var GMScene) {.base.} =
    ## This method can be implemented when the scene is entered.
    discard

method gmExit*(self: var GMScene) {.base.} =
    ## This method can be implemented when the scene is left.
    discard

method gmUpdate*(self: var GMScene) {.base.} =
    ## This method must be implemented in order to update the scene.
    quit("You must override this method: gm")

method gmDraw*(self: var GMScene) {.base.} =
    ## This method must be implemented in order to draw the scene.
    quit("You must override this method: gm")

method gmCustom*(self: var GMScene, data: JsonNode) {.base.} =
    ## This method can be implemented to send user defined data.
    discard

# GMSceneManager:

proc gmFindSceneIndex(self: GMSceneManager): uint32 =
    # TODO: get index of given scene name
    return 0

proc gmUpdate*(self: var GMSceneManager) =
    self.scenes[self.currentScene].gmUpdate()

proc gmDraw*(self: var GMSceneManager) =
    self.scenes[self.currentScene].gmDraw()

proc gmAddScene*(self: var GMSceneManager, scene: GMScene) =
    self.scenes.add(scene)

# TODO: remove scene, change to scene, replace scene, push and change scene,
# pop and change scene, send custom message

proc gmInitSceneManager*(): GMSceneManager =
    result.scenes = @[]
    result.sceneStack = @[]
    result.currentScene = 0

