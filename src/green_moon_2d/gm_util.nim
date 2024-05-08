## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This file contains some utility functions
##

from std/strformat import fmt

proc assert2*[T](expected: T, actual: T, message: string) =
    assert(expected == actual, fmt("{message} doesn't match {expected} != {actual}"))


