## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This file contains the test case for the math module.
##

# Nim std imports
#import std/json
from std/strformat import fmt

# Local imports
#import green_moon_2d/gm_scene
#import green_moon_2d/gm_json
import green_moon_2d/gm_math

proc collideRectangle1(p: GMVec2D, inside: bool = false) =
    let p1 = GMVec2D(x:3.0, y:2.0)
    let p2 = GMVec2D(x:10.0, y:6.0)
    let r = GMRectangle(p1:p1, p2:p2)

    if inside:
        assert(gmIsColliding(r, p))
    else:
        assert(not gmIsColliding(r, p))

proc collideCircle1(p: GMVec2D, inside: bool = false) =
    let m = GMVec2D(x: 6.0, y:6.0)
    let c = GMCircle(m:m, r:2.0)

    if inside:
        assert(gmIsColliding(c, p))
    else:
        assert(not gmIsColliding(c, p))

proc test1_collideRectanglePoint1() =
    let p = GMVec2D(x:14.0, y:9.0)
    collideRectangle1(p)

proc test2_collideRectanglePoint2() =
    let p = GMVec2D(x:1.0, y:4.0)
    collideRectangle1(p)

proc test3_collideRectanglePoint3() =
    let p = GMVec2D(x:14.0, y:3.0)
    collideRectangle1(p)

proc test4_collideRectanglePoint4() =
    let p = GMVec2D(x:6.0, y:10.0)
    collideRectangle1(p)

proc test5_collideRectanglePoint5() =
    let p = GMVec2D(x:7.0, y:1.0)
    collideRectangle1(p)

proc test6_collideRectanglePoint6() =
    let p = GMVec2D(x:6.0, y:4.0)
    collideRectangle1(p, true)

proc test7_collideCirclePoint1() =
    let p = GMVec2D(x:15.0, y:10.0)
    collideCircle1(p)

proc test8_collideCirclePoint2() =
    let p = GMVec2D(x:6.0, y:7.0)
    collideCircle1(p, true)

when isMainModule:
    test1_collideRectanglePoint1()
    test2_collideRectanglePoint2()
    test3_collideRectanglePoint3()
    test4_collideRectanglePoint4()
    test5_collideRectanglePoint5()
    test6_collideRectanglePoint6()
    test7_collideCirclePoint1()
    test8_collideCirclePoint2()



