## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This module contains the code for all the resource handling.
##

# Nim std imports
import std/json

# Local imports
import gm_configuration

type
    GMResourceManager = object
        stuff: bool

var GMGlobResources: GMResourceManager

proc gmProcessResources(data: string): GMResourceManager =
    let jsonData = parseJson(data)
    result.stuff = true

proc gmLoadResources*() =
    ## Load and initialize the resources from the given file name (format: JSON).
    let filename = gmGetResourcesFilename()
    let inFile = open(filename, mode = fmRead)
    let data = inFile.readAll()
    inFile.close()

    GMGlobResources = gmProcessResources(data)

# Test cases

