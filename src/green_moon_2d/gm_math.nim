## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it uses Naylib (Raylib) internally.
##
## This module contains the code for math related data structures and algorithms.
##


# Nim std imports
#import std/json
#import std/sets
#import std/options
#import std/algorithm
import std/math

from std/strformat import fmt

# Local imports
#import gm_log
#import gm_json

type
    GMVec2D* = object
        x*: float32
        y*: float32

    GMRectangle* = object
        p1*: GMVec2D
        p2*: GMVec2D

    GMCircle* = object
        m*: GMVec2D
        r*: float32

proc gmDist*(p1: GMVec2D, p2: GMVec2D): float32 =
    let x = p2.x - p1.x
    let y = p2.y - p1.y
    return hypot(x, y)

proc gmIsColliding*(r: GMRectangle, p: GMVec2D): bool =
    return (r.p1.x <= p.x) and (r.p2.x >= p.x) and (r.p1.y <= p.y) and (r.p2.y >= p.y)

proc gmIsColliding*(c: GMCircle, p: GMVec2D): bool =
    let d = gmDist(c.m, p)
    return d <= c.r

