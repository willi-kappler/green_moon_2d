# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines all math related functions and classes.
"""

import math
import enum

from typing import Self


class GMAlignment(enum.Enum):
    TOP_LEFT = enum.auto()
    TOP_CENTER = enum.auto()
    TOP_RIGHT = enum.auto()

    MID_LEFT = enum.auto()
    MID_CENTER = enum.auto()
    MID_RIGHT = enum.auto()

    BTM_LEFT = enum.auto()
    BTM_CENTER = enum.auto()
    BTM_RIGHT = enum.auto()


class GMRepetition(enum.Enum):
    """
    This class defins the various repetition types:
    FIXED: Does nothing.
    FORWARD: Runs something forward once.
    BACKWARD: Runs something backward once.
    FORWARD_LOOP: Runs something forward in a loop.
    BACKWARD_LOOP: Runs something backward in a loop.
    PINGPONG_F: Runs something forward once and then backward once and then repeats.
    """

    FIXED = enum.auto()
    FORWARD = enum.auto()
    BACKWARD = enum.auto()
    FORWARD_LOOP = enum.auto()
    BACKWARD_LOOP = enum.auto()
    PINGPONG_F = enum.auto()
    PINGPONG_B = enum.auto()

    @staticmethod
    def from_str(repetition: str) -> "GMRepetition":
        match repetition:
            case "fixed":
                return GMRepetition.FIXED
            case "forward":
                return GMRepetition.FORWARD
            case "backward":
                return GMRepetition.BACKWARD
            case "loop_forward":
                return GMRepetition.FORWARD_LOOP
            case "loop_backward":
                return GMRepetition.BACKWARD_LOOP
            case "ping_pong_forward":
                return GMRepetition.PINGPONG_F
            case "ping_pong_backward":
                return GMRepetition.PINGPONG_B
            case _:
                raise ValueError(f"GMResources, Unknown repetition: {repetition}")


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

    @classmethod
    def from_tuple(cls, pos: tuple[float, float]):
        return cls(pos[0], pos[1])

    def __eq__(self, other) -> bool:
        """
        Compares this vector to the other element wise.

        :param other: The other vector to compare to.
        :return: True if they are the same, otherwise False.
        :rtype: bool
        """

        if isinstance(other, GMVec2D):
            return math.isclose(self.x, other.x) and math.isclose(self.y, other.y)
        else:
            return False

    def __repr__(self) -> str:
        return f"GMVec2D({self.x}, {self.y})"

    def __add__(self, other: Self | tuple[float, float]) -> Self:
        """
        Adds this vector to another (or a tuple) and returns a new vector.

        :param other: The other vector or tuple.
        :return: The new vector as the sum of this and the other vector.
        :rtype: GMVec2D
        """

        match other:
            case GMVec2D(x=ox, y=oy):
                x = self.x + ox
                y = self.y + oy
                cls = type(self)
                return cls(x, y)
            case (tx, ty):
                x = self.x + tx
                y = self.y + ty
                cls = type(self)
                return cls(x, y)
            case _:
                raise ValueError(f"Type of other must be a GMVec2D or a tuple of floats: {other}")

    def add2(self, other: Self | tuple[float, float]) -> None:
        """
        Adds the values of other to this vector.

        :param other: Can be a GMVec2D or a tuple of floats.
        """

        match other:
            case GMVec2D(x=ox, y=oy):
                self.x += ox
                self.y += oy
            case (tx, ty):
                self.x += tx
                self.y += ty
            case _:
                raise ValueError(f"Type of other must be a GMVec2D or a tuple of floats: {other}")

    def __mul__(self, other: float) -> Self:
        """
        Multiplies a value with the x and y component of this vector, returns a new vector.
        :param other: The factor to multiply with, float.
        :return: The new scaled vector.
        :rtype: GMVec2D
        """

        x = self.x * other
        y = self.y * other
        cls = type(self)
        return cls(x, y)


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

        self.cx: float = cx
        self.cy: float = cy
        self.r: float = radius

    def __eq__(self, other) -> bool:
        """
        Compares this circle to another.

        :param other: The other circle to compare to.
        :return: True if all the values are equal (center + radius).
        :rtype: bool
        """

        if isinstance(other, GMCircle):
            return math.isclose(self.cx, other.cx) and math.isclose(self.cy, other.cy) and math.isclose(self.r, other.r)
        else:
            return False

    def __repr__(self) -> str:
        return f"GMCircle({self.cx}, {self.cy}, {self.r})"

    def __add__(self, other: GMVec2D | tuple[float, float]) -> Self:
        """
        Adds a vector (or tuple) to this circle and moved the center accordingly.

        :param other: The vector (or tuple) to be added to the center.
        :return: A new circle moved by the vector.
        :rtype: GMCircle
        """

        match other:
            case GMVec2D(x=ox, y=oy):
                cx = self.cx + ox
                cy = self.cy + oy
                cls = type(self)
                return cls(cx, cy, self.r)
            case (tx, ty):
                cx = self.cx + tx
                cy = self.cy + ty
                cls = type(self)
                return cls(cx, cy, self.r)
            case _:
                raise ValueError(f"Other must be a GMVec2D or a tuple of floats: {other}")

    def add2(self, other: GMVec2D | tuple[float, float]) -> None:
        """
        Add other to the component of this circle and moves the center.

        :param other: Can be a vector or a tuple of floats.
        """

        match other:
            case GMVec2D(x=ox, y=oy):
                self.cx += ox
                self.cy += oy
            case (tx, ty):
                self.cx += tx
                self.cy += ty
            case _:
                raise ValueError(f"Other must be a GMVec2D or a tuple of floats: {other}")

    def inside(self, point: GMVec2D | tuple[float, float]) -> bool:
        """
        Check if a point (vector or tuple) is inside a circle.

        :param point: A vector or a tuple of two floats.
        :return: True if point is inside this circle, False if outside.
        :rtype: bool
        """

        match point:
            case GMVec2D(x=px, y=py):
                dx = self.cx - px
                dy = self.cy - py
                d = math.hypot(dx, dy)
                return d <= self.r
            case (tx, ty):
                dx = self.cx - tx
                dy = self.cy - ty
                d = math.hypot(dx, dy)
                return d <= self.r
            case _:
                raise ValueError(f"Point must be a GMVec2D or a tuple of floats: {point}")


def gm_orbit_circle1(x: float, y: float, r: float, angle: float) -> tuple[float, float]:
    """
    Returns the coordinates of a point orbiting a circle.

    :param x: X position of the circle center.
    :param y: Y position of the circle center.
    :param r: The radius of the circle.
    :param angle: The angle of the point around the circle.
    :return: A tuple of floats with the x and y coordinates of the point.
    :rtype: tuple[float, float]
    """

    rad = math.radians(angle)
    px = x + (r * math.cos(rad))
    py = y + (r * math.sin(rad))
    return (px, py)


def gm_orbit_circle2(pos: GMVec2D, r: float, angle: float) -> tuple[float, float]:
    return gm_orbit_circle1(pos.x, pos.y, r, angle)


def gm_orbit_circle3(circle: GMCircle, angle: float) -> tuple[float, float]:
    return gm_orbit_circle1(circle.cx, circle.cy, circle.r, angle)




