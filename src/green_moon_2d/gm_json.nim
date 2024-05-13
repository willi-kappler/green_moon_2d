## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This file contains utility function for JSON value extraction
##

# Nim std imports
import std/json
import std/sets
import std/sequtils
#from std/strformat import fmt

proc gmGetBool*(node: JsonNode, name: string, default: bool = false): bool =
    return node{name}.getBool(default)

proc gmGetUint8*(node: JsonNode, name: string, default: uint8 = 0): uint8 =
    return uint8(node{name}.getInt(int(default)))

proc gmGetInt32*(node: JsonNode, name: string, default: int32 = 0): int32 =
    return int32(node{name}.getInt(int(default)))

proc gmGetUint32*(node: JsonNode, name: string, default: uint32 = 0): uint32 =
    return uint32(node{name}.getInt(int(default)))

proc gmGetFloat32*(node: JsonNode, name: string, default: float32 = 0.0): float32 =
    return float32(node{name}.getFloat(float32(default)))

proc gmGetF32F32*(node: JsonNode, name: string, default: (float32, float32) = (0.0, 0.0)): (float32, float32) =
    let elems = node[name].getElems()
    let a = float32(elems[0].getFloat())
    let b = float32(elems[1].getFloat())
    return (a, b)

proc gmGetString*(node: JsonNode, name: string, default: string = ""): string =
    return node{name}.getStr(default)

proc gmGetHashSetString*(node: JsonNode, name: string, default: HashSet[string] = initHashSet[string]()): HashSet[string] =
    result = default

    if node.contains(name):
        let elements = node[name].getElems()
        #echo(fmt("gmGetHashSetString(), elements: {elements}"))
        for e in elements:
            result.incl(e.getStr())

proc gmGetNode*(node: JsonNode, name: string, default: JsonNode = newJNull()): JsonNode =
    result = default
    if node.contains(name):
        result = node[name]

proc gmGetNodes*(node: JsonNode, name: string, default: seq[JsonNode] = @[]): seq[JsonNode] =
    return node{name}.getElems(default)

