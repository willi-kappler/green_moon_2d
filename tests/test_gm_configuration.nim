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
import green_moon_2d/gm_util

proc checkProperties(node: JsonNode = newJNull()) =
    let logFilename = gmGetString(node, "LogFilename", "game.log")
    let fps = gmGetUint32(node, "FPS", 60)
    let screenWidth = gmGetUint32(node, "ScreenWidth", 800)
    let screenHeight = gmGetUint32(node, "ScreenHeight", 600)
    let windowTitle = gmGetString(node, "WindowTitle", "Made with GreenMoon2D")
    let resourcesFilename = gmGetString(node, "ResourcesFilename", "resources.json")

    assert2(gmGetLogFilename(), logFilename, "LogFilename")
    assert2(gmGetFPS(), fps, "FPS")
    assert2(gmGetScreenWidth(), screenWidth, "ScreenWidth")
    assert2(gmGetScreenHeight(), screenHeight, "ScreenHeight")
    assert2(gmGetWindowTitle(), windowTitle, "WindowTitle")
    assert2(gmGetResourcesFilename(), resourcesFilename, "ResourcesFilename")

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
    checkProperties(%*{"LogFilename": "test4.log"})

proc test5_validateJSON() =
    let inputConfig = """{"fps": 50}"""
    let filename = "tests/config5.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))
    checkProperties(%*{"FPS": 50})

proc test6_validateJSON() =
    let inputConfig = """{"screenWidth": 1024}"""
    let filename = "tests/config6.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))
    checkProperties(%*{"ScreenWidth": 1024})

proc test7_validateJSON() =
    let inputConfig = """{"screenHeight": 768}"""
    let filename = "tests/config7.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))
    checkProperties(%*{"ScreenHeight": 768})

proc test8_validateJSON() =
    let inputConfig = """{"windowTitle": "Test8"}"""
    let filename = "tests/config8.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))
    checkProperties(%*{"WindowTitle": "Test8"})

proc test9_validateJSON() =
    let inputConfig = """{"resources": "some_file.json"}"""
    let filename = "tests/config9.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))
    checkProperties(%*{"ResourcesFilename": "some_file.json"})

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

