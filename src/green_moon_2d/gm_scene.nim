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
import std/options
from std/strformat import fmt

# Local imports
import gm_log

type
    GMScene* = ref object of RootObj
        name: string

    GMSceneManager = object
        scenes: seq[GMScene]
        sceneStack: seq[string]
        currentScene: uint32

var GMGlobScenes: GMSceneManager

# GMScene:

method gmEnter*(self: var GMScene) {.base.} =
    ## This method can be implemented when the scene is entered.
    discard

method gmDraw*(self: var GMScene) {.base.} =
    ## This method must be implemented in order to draw the scene.
    quit("You must override this method: gm")

method gmUpdate*(self: var GMScene) {.base.} =
    ## This method must be implemented in order to update the scene.
    quit("You must override this method: gm")

method gmCustom*(self: var GMScene, data: JsonNode): JsonNode {.base.} =
    ## This method can be implemented to send or receive user defined data.
    return newJNull()

# GMSceneManager:

proc gmFindSceneIndex(name: string): Option[uint32] =
    ## Return the index of a given scene (by name) or none if no such scene was found.
    for (i, s) in GMGlobScenes.scenes.pairs():
        if s.name == name:
            return some(uint32(i))

    return none(uint32)

proc gmDraw*() =
    ## Calls the gmDraw() method on the current active scene.
    GMGlobScenes.scenes[GMGlobScenes.currentScene].gmDraw()

proc gmUpdate*() =
    ## Calls the gmUpdate() method on the current active scene.
    GMGlobScenes.scenes[GMGlobScenes.currentScene].gmUpdate()

proc gmAddScene*(scene: GMScene) =
    ## Adds a new scene to the list of scenes.
    let idx = gmFindSceneIndex(scene.name)

    if idx.isSome():
        gmLoggerError(fmt("Can't add scene: {scene.name}, a scene with that name already exists!"))
    else:
        GMGlobScenes.scenes.add(scene)

proc gmRemoveScene*(name: string) =
    ## Removes a scene with the given name.
    let idx = gmFindSceneIndex(name)

    if idx.isSome():
        let i = idx.get()
        if i != GMGlobScenes.currentScene:
            # Remove scene
            GMGlobScenes.scenes.del(i)
        else:
            gmLoggerError(fmt("Can't remove current active scene: {name}, index: {i}!"))
    else:
        gmLoggerError(fmt("Can't remove scene: {name}, it was not found!"))

proc gmChangeScene*(name: string) =
    ## Changes the active scnene to the one with the given name.
    let idx = gmFindSceneIndex(name)

    if idx.isSome():
        let i = idx.get()
        if i != GMGlobScenes.currentScene:
            GMGlobScenes.currentScene = i
            GMGlobScenes.scenes[i].gmEnter()
        else:
            gmLoggerWarn(fmt("Scene is already active: {name}, index: {i}!"))
    else:
        gmLoggerError(fmt("Can't change to scene: {name}, it was not found!"))

proc gmReplaceScene*(name: string, scene: GMScene) =
    ## Replace the scenen given by the name with the new given one.
    let idx = gmFindSceneIndex(name)

    if idx.isSome():
        let i = idx.get()
        GMGlobScenes.scenes[i] = scene
    else:
        gmLoggerError(fmt("Can't replace scene: {name}, it was not found!"))

proc gmPushAndChangeScene*(name: string) =
    ## Pushes the current scene onto a stack and change to the given scene.
    ## This is useful if for example you want to show a menu during the game.
    let idx = gmFindSceneIndex(name)

    if idx.isSome():
        let i = idx.get()
        if i != GMGlobScenes.currentScene:
            let currentSceneName = GMGlobScenes.scenes[GMGlobScenes.currentScene].name
            GMGlobScenes.sceneStack.add(currentSceneName)
            GMGlobScenes.currentScene = i
            GMGlobScenes.scenes[i].gmEnter()
        else:
            gmLoggerWarn(fmt("Can't push scene, it is already active: {name}, index: {i}!"))
    else:
        gmLoggerError(fmt("Can't push scene: {name}, it was not found!"))

proc gmPopAndChangeScene*() =
    ## Pops the last scene from the stack and changes to it.
    if GMGlobScenes.sceneStack.len() > 0:
        let previousSceneName = GMGlobScenes.sceneStack.pop()
        gmChangeScene(previousSceneName)
    else:
        gmLoggerError(fmt("Can't pop scene: scene stack is empty"))

proc gmCustom*(name: string, data: JsonNode): JsonNode =
    ## Sends a message with custom data to the given scene.
    let idx = gmFindSceneIndex(name)

    if idx.isSome():
        let i = idx.get()
        return GMGlobScenes.scenes[i].gmCustom(data)
    else:
        gmLoggerError(fmt("Can't send cutom data to scene: {name}, it was not found!"))
        return newJNull()

proc gmInitSceneManager*() =
    GMGlobScenes.scenes = @[]
    GMGlobScenes.sceneStack = @[]
    GMGlobScenes.currentScene = 0

