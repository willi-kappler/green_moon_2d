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

# Local imports
import green_moon_2d/gm_configuration

proc test1_loadConfiguration() =
    let filename = "does_not_exists.json"

    doAssertRaises IOError:
         gmLoadConfiguration(filename)

proc test2_loadConfiguration() =
    let filename = "tests/config1.json"
    gmLoadConfiguration(filename)

    assert(gmGetLogFilename() == "test1.log")
    assert(gmGetFPS() == 58)
    assert(gmGetScreenWidth() == 800)
    assert(gmGetscreenHeight() == 600)
    assert(gmGetWindowTitle() == "TestConfig")
    assert(gmGetResourcesFilename() == "resources.json")

proc test3_validateJSON() =
    let inputConfig = "{}"
    let filename = "tests/config3.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    assert(gmGetLogFilename() == "game.log")
    assert(gmGetFPS() == 60)
    assert(gmGetScreenWidth() == 800)
    assert(gmGetscreenHeight() == 600)
    assert(gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(gmGetResourcesFilename() == "resources.json")

proc test4_validateJSON() =
    let inputConfig = """{"logFilename": "test4.log"}"""
    let filename = "tests/config4.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    assert(gmGetLogFilename() == "test4.log")
    assert(gmGetFPS() == 60)
    assert(gmGetScreenWidth() == 800)
    assert(gmGetscreenHeight() == 600)
    assert(gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(gmGetResourcesFilename() == "resources.json")

proc test5_validateJSON() =
    let inputConfig = """{"fps": 50}"""
    let filename = "tests/config5.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    assert(gmGetLogFilename() == "game.log")
    assert(gmGetFPS() == 50)
    assert(gmGetScreenWidth() == 800)
    assert(gmGetscreenHeight() == 600)
    assert(gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(gmGetResourcesFilename() == "resources.json")

proc test6_validateJSON() =
    let inputConfig = """{"screenWidth": 1024}"""
    let filename = "tests/config6.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    assert(gmGetLogFilename() == "game.log")
    assert(gmGetFPS() == 60)
    assert(gmGetScreenWidth() == 1024)
    assert(gmGetscreenHeight() == 600)
    assert(gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(gmGetResourcesFilename() == "resources.json")

proc test7_validateJSON() =
    let inputConfig = """{"screenHeight": 768}"""
    let filename = "tests/config7.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    assert(gmGetLogFilename() == "game.log")
    assert(gmGetFPS() == 60)
    assert(gmGetScreenWidth() == 800)
    assert(gmGetscreenHeight() == 768)
    assert(gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(gmGetResourcesFilename() == "resources.json")

proc test8_validateJSON() =
    let inputConfig = """{"windowTitle": "Test8"}"""
    let filename = "tests/config8.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    assert(gmGetLogFilename() == "game.log")
    assert(gmGetFPS() == 60)
    assert(gmGetScreenWidth() == 800)
    assert(gmGetscreenHeight() == 600)
    assert(gmGetWindowTitle() == "Test8")
    assert(gmGetResourcesFilename() == "resources.json")

proc test9_validateJSON() =
    let inputConfig = """{"resources": "some_file.json"}"""
    let filename = "tests/config9.json"
    writeFile(filename, inputConfig)
    gmLoadConfiguration(filename)
    removeFile(Path(filename))

    assert(gmGetLogFilename() == "game.log")
    assert(gmGetFPS() == 60)
    assert(gmGetScreenWidth() == 800)
    assert(gmGetscreenHeight() == 600)
    assert(gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(gmGetResourcesFilename() == "some_file.json")

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

