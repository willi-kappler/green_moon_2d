## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This module contains the configuration for the library / the game.
##

# Nim std imports
import std/json

type
    GMConfiguration = object
        ## Configuration options for GreenMoon2D.
        logFilename: string
        fps: uint32
        windowTitle: string
        screenWidth: uint32
        screenHeight: uint32
        resources: string

var GMGlobConfig: GMConfiguration

proc gmGetLogFilename*(): string =
    return GMGlobConfig.logFilename

proc gmGetFPS*(): uint32 =
    return GMGlobConfig.fps

proc gmGetWindowTitle*(): string =
    return GMGlobConfig.windowTitle

proc gmGetScreenWidth*(): uint32 =
    return GMGlobConfig.screenWidth

proc gmGetScreenHeight*(): uint32 =
    return GMGlobConfig.screenHeight

proc gmGetResourcesFilename*(): string =
    return GMGlobConfig.resources

proc gmValidateConfiguration(jsonString: string): GMConfiguration =
    ## Checks if the given json string contains a valid configuration
    let jsonConfig = parseJson(jsonString)

    result.logFilename = jsonConfig{"logFilename"}.getStr("")
    if result.logFilename.len() == 0:
        result.logFilename = "game.log"

    result.fps = uint32(jsonConfig{"fps"}.getInt(0))
    if result.fps == 0:
        result.fps = 60

    result.windowTitle = jsonConfig{"windowTitle"}.getStr("")
    if result.windowTitle.len() == 0:
        result.windowTitle = "Made with GreenMoon2D"

    result.screenWidth = uint32(jsonConfig{"screenWidth"}.getInt(0))
    if result.screenWidth == 0:
        result.screenWidth = 800

    result.screenHeight = uint32(jsonConfig{"screenHeight"}.getInt(0))
    if result.screenHeight == 0:
        result.screenHeight = 600

    result.resources = jsonConfig{"resources"}.getStr("")
    if result.resources.len() == 0:
        result.resources = "resources.json"

proc gmLoadConfiguration*(filename: string) =
    ## Load the configuration from the given file name (format: JSON).
    let inFile = open(filename, mode = fmRead)
    let data = inFile.readAll()
    inFile.close()
    GMGlobConfig = gmValidateConfiguration(data)

# Test cases




