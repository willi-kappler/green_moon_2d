## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This file contains the test case for the configuration
##

# Nim std imports
from std/files import removeFile
from std/paths import Path
from std/strformat import fmt

import std/json

# Local imports
import green_moon_2d/gm_configuration
import green_moon_2d/gm_json

proc checkProperties(node: JsonNode = newJNull()) =
    let logFilename = gmGetString(node, "LogFilename", "game.log")
    let fps = gmGetUint32(node, "FPS", 60)
    let screenWidth = gmGetUint32(node, "ScreenWidth", 800)
    let screenHeight = gmGetUint32(node, "ScreenHeight", 600)
    let windowTitle = gmGetString(node, "WindowTitle", "Made with GreenMoon2D")
    let resourcesFilename = gmGetString(node, "ResourcesFilename", "resources.json")

    assert(gmGetLogFilename() == logFilename, fmt("LogFilename doesn't match: {logFilename}"))
    assert(gmGetFPS() == fps, fmt("FPS doesn't match: {fps}"))
    assert(gmGetScreenWidth() == screenWidth, fmt("ScreenWidth doesn't match: {screenWidth}"))
    assert(gmGetscreenHeight() == screenHeight, fmt("ScreenHeight doesn't match: {screenHeight}"))
    assert(gmGetWindowTitle() == windowTitle, fmt("WindowTitle doesn't match: {windowTitle}"))
    assert(gmGetResourcesFilename() == resourcesFilename, fmt("ResourcesFilename doesn't match: {resourcesFilename}"))

proc checkStringProperty(name: string, value: string) =
    # Checks that the given property of the configuration is equal
    # to the given value.

    # Default values:
    var logFilename = "game.log"
    var windowTitle = "Made with GreenMoon2D"
    var resourcesFilename = "resources.json"

    if name == "LogFilename":
        logFilename = value
    elif name == "WindowTitle":
        windowTitle = value
    elif name == "ResourcesFilename":
        resourcesFilename = value
    else:
        raise newException(ValueError, fmt("Unknown property name: {name}"))

    assert(gmGetLogFilename() == logFilename)
    assert(gmGetFPS() == 60)
    assert(gmGetScreenWidth() == 800)
    assert(gmGetscreenHeight() == 600)
    assert(gmGetWindowTitle() == windowTitle)
    assert(gmGetResourcesFilename() == resourcesFilename)

proc checkIntProperty(name: string, value: uint32) =
    # Checks that the given property of the configuration is equal
    # to the given value.

    # Default values:
    var fps: uint32 = 60
    var screenWidth: uint32 = 800
    var screenHeight: uint32 = 600

    if name == "FPS":
        fps = value
    elif name == "ScreenWidth":
        screenWidth = value
    elif name == "ScreenHeight":
        screenHeight = value
    else:
        raise newException(ValueError, fmt("Unknown property name: {name}"))

    assert(gmGetLogFilename() == "game.log")
    assert(gmGetFPS() == fps)
    assert(gmGetScreenWidth() == screenWidth)
    assert(gmGetscreenHeight() == screenHeight)
    assert(gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(gmGetResourcesFilename() == "resources.json")

proc test1_loadConfiguration() =
    let filename = "does_not_exists.json"

    doAssertRaises IOError:
         gmLoadConfiguration(filename)

proc test2_loadConfiguration() =
    let filename = "tests/config1.json"
    gmLoadConfiguration(filename)
    checkProperties(%*{"LogFilename": "test1.log", "FPS": 58,
                       "ScreenWidth": 1024, "ScreenHeight": 768,
                       "WindowTitle": "TestConfig"})

proc test3_validateJSON() =
    let inputConfig = "{}"
    let filename = "tests/config3.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))
    checkProperties()

proc test4_validateJSON() =
    let inputConfig = """{"logFilename": "test4.log"}"""
    let filename = "tests/config4.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    checkStringProperty("LogFilename", "test4.log")

proc test5_validateJSON() =
    let inputConfig = """{"fps": 50}"""
    let filename = "tests/config5.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    checkIntProperty("FPS", 50)

proc test6_validateJSON() =
    let inputConfig = """{"screenWidth": 1024}"""
    let filename = "tests/config6.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    checkIntProperty("ScreenWidth", 1024)

proc test7_validateJSON() =
    let inputConfig = """{"screenHeight": 768}"""
    let filename = "tests/config7.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    checkIntProperty("ScreenHeight", 768)

proc test8_validateJSON() =
    let inputConfig = """{"windowTitle": "Test8"}"""
    let filename = "tests/config8.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    checkStringProperty("WindowTitle", "Test8")

proc test9_validateJSON() =
    let inputConfig = """{"resources": "some_file.json"}"""
    let filename = "tests/config9.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    checkStringProperty("ResourcesFilename", "some_file.json")

when isMainModule:
    test1_loadConfiguration()
    test2_loadConfiguration()
    test3_validateJSON()
    test4_validateJSON()
    test5_validateJSON()
    test6_validateJSON()
    test7_validateJSON()
    test8_validateJSON()
    test9_validateJSON()

