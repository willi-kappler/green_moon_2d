# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines all math related functions and classes.
"""

from typing import Self


class GMVec2D:
    """
    This class defines a 2D vector.
    """

    __slots__ = ("x", "y")

    def __init__(self, x: float, y: float):
        """
        :param x: The x component of the 2d vector.
        :param y: The y component of the 2d vector.
        """
        self.x: float = x
        self.y: float = y

    def __add__(self, other: Self) -> Self:
        """
        Adds this vector to another and returns a new vector.

        :param other: The other vector.
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

    def add2(self, other: Self | tuple[float, float]):
        """
        Adds the values of other to this vector.
        :param other: Can be a GMVec2D or a tuple of floats.
        """

        if isinstance(other, GMVec2D):
            self.x += other.x
            self.y += other.y
        elif isinstance(other, tuple):
            self.x += other[0]
            self.y += other[1]
        else:
            raise ValueError(f"Type of other must be a GMVec2D or a tuple of floats: {other}")


class GMCircle:
    """
    This class defines a circle.
    """

    __slots__ = ("cx", "cy", "r")

    def __init__(self, cx: float, cy: float, radius: float):
        """
        :param cx: The x component of the circle center.
        :param cy: The y component of the circle center.
        :param radius: The radius of the circle.
        """

        self.cx = cx
        self.cy = cy
        self.r = radius

    def __add__(self, other: GMVec2D) -> Self:
        """
        Adds a vector to this circle and moved the center accordingly.

        :param other: The vector to be added to the center.
        :return: A new circle moved by the vector.
        :rtype: GMCircle
        """

        cx = self.cx + other.x
        cy = self.cy + other.y

        cls = type(self)
        return cls(cx, cy, self.r)

    def add2(self, other: GMVec2D | tuple[float, float]):
        """
        Add other to the component of this circle and moves the center.

        :param other: Can be a vector or a tuple of floats.
        """

        if isinstance(other, GMVec2D):
            self.cx += other.x
            self.cy += other.y
        elif isinstance(other, tuple):
            self.cx += other[0]
            self.cy += other[1]
        else:
            raise ValueError(f"Other must be a GMVec2D or a tuple of floats: {other}")

    def __eq__(self, other) -> bool:
        """
        Compares this circle to another.

        :param other: The other circle to compare to.
        :return: True if all the values are equal (center + radius).
        :rtype: bool
        """

        if isinstance(other, GMCircle):
            return (self.cx == other.cx) and (self.cy == other.cy) and (self.r == other.r)
        else:
            return False




