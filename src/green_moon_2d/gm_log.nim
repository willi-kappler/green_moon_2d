## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This module contains the code for the logger
##

# Nim std imports
#from std/strformat import fmt

type
    GMLogger = object
        debugLevel: uint8

var GMGlobLogger: GMLogger

proc gmLoggerDebug*(message: string) =
    discard

proc gmLoggerInfo*(message: string) =
    discard

proc gmLoggerWarn*(message: string) =
    discard

proc gmLoggerError*(message: string) =
    discard

proc gmSetDebugLevel*(level: uint8) =
    GMGlobLogger.debugLevel = level

proc gmGetDebugLevel*(): uint8 =
    return GMGlobLogger.debugLevel

proc gm_initLogger*() =
    GMGlobLogger = GMLogger(debugLevel: 0)

