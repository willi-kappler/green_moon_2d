# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMContext class which is passed to all the update()
and draw() methods.
"""

from typing import Any


class GMContext:
    """
    This class contains acess to the scene manager and the state of the
    game engine. It also allows you to store data that is used across
    multiple scenes.
    """

    def __init__(self):
        self.game_property = {}

    def set_property(self, name: str, val: Any) -> None:
        self.game_property[name] = val
