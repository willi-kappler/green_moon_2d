# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines messages used in the game engine.
"""


import green_moon_2d.gm_interfaces as gmi


class GMSceneMessage:
    def __init__(self, kind: str, scene: None | gmi.GMSceneInterface = None, scene_name: str = ""):
        self.kind: str = kind
        self.scene = scene
        self.scene_name = scene_name


