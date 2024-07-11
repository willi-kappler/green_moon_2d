# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines all math related functions and objects
"""

from typing import Self


class GMVec2D:
    """
    This class defines a 2D vector.
    """

    __slots__ = ("x", "y")

    def __init__(self, x: float = 0.0, y: float = 0.0):
        self.x: float = x
        self.y: float = y

    def __add__(self, other: Self) -> Self:
        """
        Adds this vector to another and returns a new vector.

        :param GMVec2D other: The other vector.
        :return: The new vector as the sum of this and the other vector.
        :rtype: GMVec2D
        """
        x = self.x + other.x
        y = self.y + other.y
        cls = type(self)
        return cls(x, y)

    def __eq__(self, other) -> bool:
        """
        Compares this vector to the other element wise.

        :param other: The other vector to compare to.
        :return: True if they are the same, otherwise False.
        :rtype: bool
        """

        if isinstance(other, GMVec2D):
            return (self.x == other.x) and (self.y == other.y)
        else:
            return False

