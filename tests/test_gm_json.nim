## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This file contains the test case for json helper functions
##

# Nim std imports
import std/json
from std/strformat import fmt

# Local imports
import green_moon_2d/gm_json

proc test1_gmGetBool() =
    let j1 = newJObject()
    var j2 = newJObject()
    var j3 = newJObject()

    j2["myBool"] = newJBool(true)
    j3["myBool"] = newJBool(false)

    assert(gmGetBool(j1, "myBool") == false)
    assert(gmGetBool(j1, "myBool", true) == true)

    assert(gmGetBool(j2, "myBool") == true)
    assert(gmGetBool(j2, "myBool2") == false)
    assert(gmGetBool(j2, "myBool2", true) == true)

    assert(gmGetBool(j3, "myBool") == false)
    assert(gmGetBool(j3, "myBool2") == false)
    assert(gmGetBool(j3, "myBool2", true) == true)

when isMainModule:
    test1_gmGetBool()
    # test2_gmGetUint8()
    # test3_gmGetInt32()
    # test4_gmGetUint32()
    # test5_gmGetFloat32()
    # test6_gmGetF32F32()
    # test7_gmGetString()
    # test8_gmGetHashSetString()
    # test9_gmGetNode()
    # test10_gmGetNodes()
    # test11_gmHashSetStringToJson()







