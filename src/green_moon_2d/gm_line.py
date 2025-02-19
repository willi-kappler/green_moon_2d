# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

# Python std library:
from typing import Any, override
from enum import Enum
from copy import deepcopy
import math

# Local imports:
from green_moon_2d.gm_object import GMObject
from green_moon_2d.gm_math import GMVec2D


class GMLineMode(Enum):
    COUNT = 0
    SPACING = 1


class GMLineRepeat(Enum):
    RAMP = 0
    REFLECT = 1


class GMLine(GMObject):
    def __init__(self, name: str, pos1: GMVec2D, pos2: GMVec2D, initial: GMObject | list[GMObject]):
        """
        :param name: The name of the line. It must be unique.
        :param pos1: The start position of the line as a 2d vector
        :param pas2: The end position of the line as a 2d vector
        :param obj: The initial object that is used to draw the line.
        """

        super().__init__(name)

        self.pos1: GMVec2D = pos1
        self.pos2: GMVec2D = pos2

        # Default values:
        self.mode = (GMLineMode.COUNT, 3)
        self.repeat: GMLineRepeat = GMLineRepeat.RAMP

        match initial:
            case GMObject():
                self.initial: list[GMObject] = [initial]
            case list():
                if len(initial) > 0:
                    for o in initial:
                        if not isinstance(o, GMObject):
                            raise ValueError("All items in list must be of type GMObject.")
                    self.initial: list[GMObject] = initial
                else:
                    raise ValueError(f"You must provide at least one object in list.")
            case _:
                raise TypeError("Initial value must be of type GMObject or list[GMObject].")

        self.create_line_objs()

    @classmethod
    def from_tuple(cls, name: str, pos1: tuple[float, float], pos2: tuple[float, float],
            initial: GMObject | list[GMObject]):
        v1: GMVec2D = GMVec2D.from_tuple(pos1)
        v2: GMVec2D = GMVec2D.from_tuple(pos2)
        return cls(name, v1, v2, initial)

    @classmethod
    def from_coords(cls, name: str, x1: float, y1: float, x2: float, y2: float,
            initial: GMObject | list[GMObject]):
        v1: GMVec2D = GMVec2D(x1, y1)
        v2: GMVec2D = GMVec2D(x2, y2)
        return cls(name, v1, v2, initial)

    @override
    def draw(self) -> None:
        for o in self.line_objs:
            o.draw()

    @override
    def update(self, dt: float) -> None:
        for o in self.line_objs:
            o.update(dt)

    @override
    def send_message(self, msg: Any) -> Any:
        """
        Process messages send to this line object.

        :param msg: The actual message.
        """

        match msg:
            # TODO: add more message cases
            case _:
                # Delegate all other messages to the base class:
                super().send_message(msg)

    def set_spacing(self, spacing: float):
        if spacing > 0.0:
            self.mode = (GMLineMode.SPACING, spacing)
            self.create_line_objs()
        else:
            raise ValueError(f"Spacing must be > 0.0, but is: {spacing}.")

    def set_count(self, count: int):
        if count > 0:
            self.mode = (GMLineMode.COUNT, count)
            self.create_line_objs()
        else:
            raise ValueError(f"Count must be > 0, but is 0.")

    def create_line_objs(self):
        n: int = 0
        sx: float = self.pos1.x
        sy: float = self.pos1.y
        lx: float = sx - self.pos2.x
        ly: float = self.pos1.y - self.pos2.y

        match self.mode:
            case (GMLineMode.COUNT, c):
                n: int = c
            case (GMLineMode.SPACING, s):
                l: float = math.hypot(lx, ly)
                n: int = math.floor(l / s)

        dx: float = lx / n
        dy: float = ly / n
        self.line_objs = []

        if len(self.initial) == 1:
            init_obj = self.initial[0]
            for i in range(0, n):
                o: GMObject = deepcopy(init_obj)
                o.pos.x = sx + (i * dx)
                o.pos.y = sy + (i * dy)
                self.line_objs.append(o)
        else:
            match self.repeat:
                case GMLineRepeat.RAMP:
                    i: int = 0
                    for j in range(0, n):
                        o: GMObject = deepcopy(self.initial[i])
                        o.pos.x = sx + (j * dx)
                        o.pos.y = sy + (j * dy)
                        self.line_objs.append(o)
                        i += 1
                        if i >= len(self.initial):
                            i = 0

                case GMLineRepeat.REFLECT:
                    # TODO: implement reflect
                    pass
                case _:
                    raise ValueError(f"Unknown line repeat value: {self.repeat}")


