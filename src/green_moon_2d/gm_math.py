# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines all math related functions and objects
"""

from typing import Self


class GMVec2D:
    def __init__(self, x=0.0, y=0.0):
        self.x = x
        self.y = y

    def __add__(self, other: Self) -> Self:
        x = self.x + other.x
        y = self.y + other.y
        return type(self)(x, y)
