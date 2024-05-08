## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it uses Naylib (Raylib) internally.
##
## This module contains the code for the logger
##

# Nim std imports
from std/strformat import fmt

type
    GMLogger = object
        debugLevel: uint8

var GMGlobLogger: GMLogger

proc gmLoggerDebug*(message: string) =
    # TODO: implement
    discard

proc gmLoggerInfo*(message: string) =
    # TODO: implement
    discard

proc gmLoggerWarn*(message: string) =
    # TODO: implement
    discard

proc gmLoggerError*(message: string) =
    # TODO: implement
    discard

proc gmSetDebugLevel*(level: uint8) =
    GMGlobLogger.debugLevel = level

proc gmGetDebugLevel*(): uint8 =
    return GMGlobLogger.debugLevel

proc gm_initLogger*() =
    GMGlobLogger = GMLogger(debugLevel: 0)

proc error_log_intern(filename: string, line: int, message: string, exceptn: typedesc) =
    gmLoggerError(fmt("{filename} - {line}: {message}"))
    raise newException(exceptn, message)

template error_log*(message: string, exceptn: typedesc) =
    let pos = instantiationInfo()
    error_log_intern(pos.filename, pos.line, message, exceptn)


