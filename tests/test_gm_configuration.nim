## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This file contains the test cased for the configuration
##

# Local imports
import green_moon_2d/gm_configuration

proc test1_loadConfiguration() =
    let filename = "does_not_exists.json"

    doAssertRaises IOError:
        let config1 = gmLoadConfiguration(filename)

proc test2_loadConfiguration() =
    let filename = "tests/config1.json"
    let config1 = gmLoadConfiguration(filename)

    assert(config1.gmGetFPS() == 58)
    assert(config1.gmGetScreenWidth() == 800)
    assert(config1.gmGetscreenHeight() == 600)
    assert(config1.gmGetWindowTitle() == "TestConfig")
    assert(config1.gmGetResourcesFilename() == "resources.json")

proc test3_validateJSON() =
    let inputConfig = "{}"
    let config1 = gmValidateConfiguration(inputConfig)

    assert(config1.gmGetFPS() == 60)
    assert(config1.gmGetScreenWidth() == 800)
    assert(config1.gmGetscreenHeight() == 600)
    assert(config1.gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(config1.gmGetResourcesFilename() == "resources.json")

proc test4_validateJSON() =
    let inputConfig = """{"fps": 50}"""
    let config1 = gmValidateConfiguration(inputConfig)

    assert(config1.gmGetFPS() == 50)
    assert(config1.gmGetScreenWidth() == 800)
    assert(config1.gmGetscreenHeight() == 600)
    assert(config1.gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(config1.gmGetResourcesFilename() == "resources.json")

proc test5_validateJSON() =
    let inputConfig = """{"screenWidth": 1024}"""
    let config1 = gmValidateConfiguration(inputConfig)

    assert(config1.gmGetFPS() == 60)
    assert(config1.gmGetScreenWidth() == 1024)
    assert(config1.gmGetscreenHeight() == 600)
    assert(config1.gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(config1.gmGetResourcesFilename() == "resources.json")

proc test6_validateJSON() =
    let inputConfig = """{"screenHeight": 768}"""
    let config1 = gmValidateConfiguration(inputConfig)

    assert(config1.gmGetFPS() == 60)
    assert(config1.gmGetScreenWidth() == 800)
    assert(config1.gmGetscreenHeight() == 768)
    assert(config1.gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(config1.gmGetResourcesFilename() == "resources.json")

proc test7_validateJSON() =
    let inputConfig = """{"windowTitle": "Test7"}"""
    let config1 = gmValidateConfiguration(inputConfig)

    assert(config1.gmGetFPS() == 60)
    assert(config1.gmGetScreenWidth() == 800)
    assert(config1.gmGetscreenHeight() == 600)
    assert(config1.gmGetWindowTitle() == "Test7")
    assert(config1.gmGetResourcesFilename() == "resources.json")

proc test8_validateJSON() =
    let inputConfig = """{"resources": "some_file.json"}"""
    let config1 = gmValidateConfiguration(inputConfig)

    assert(config1.gmGetFPS() == 60)
    assert(config1.gmGetScreenWidth() == 800)
    assert(config1.gmGetscreenHeight() == 600)
    assert(config1.gmGetWindowTitle() == "Made with GreenMoon2D")
    assert(config1.gmGetResourcesFilename() == "some_file.json")

when isMainModule:
    test1_loadConfiguration()
    test2_loadConfiguration()
    test3_validateJSON()
    test4_validateJSON()
    test5_validateJSON()
    test6_validateJSON()
    test7_validateJSON()
    test8_validateJSON()

