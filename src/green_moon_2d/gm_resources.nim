## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This module contains the ccode for all the resource handling.
##

# Nim std imports
import std/json

type
    GMResourceManager* = object
        stuff: bool


proc gmLoadResources*(filename: string): GMResourceManager =
    ## Load and initialize the resources from the given file name (format: JSON).
    let inFile = open(filename, mode = fmRead)
    let data = inFile.readAll()
    inFile.close()

    result.stuff = true


