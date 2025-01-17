# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override

from green_moon_2d.gm_object import GMObject
from green_moon_2d.gm_math import GMVec2D


class GMLine(GMObject):
    def __init__(self, name: str, pos1: GMVec2D, pos2: GMVec2D, obj: GMObject):
        """
        :param name: The name of the line. It must be unique.
        :param pos1: The start position of the line as a 2d vector
        :param pas2: The end position of the line as a 2d vector
        :param obj: The initial object that is used to draw the line.
        """

        super().__init__(name)

        self.pos1: GMVec2D = pos1
        self.pos2: GMVec2D = pos2
        self.initial_obj = obj
        self.line_objs = []

    @classmethod
    def from_tuple(cls, name: str, pos1: tuple[float, float], pos2: tuple[float, float], obj: GMObject):
        v1: GMVec2D = GMVec2D.from_tuple(pos1)
        v2: GMVec2D = GMVec2D.from_tuple(pos2)
        return cls(name, v1, v2, obj)

    @classmethod
    def from_coords(cls, name: str, x1: float, y1: float, x2: float, y2: float, obj: GMObject):
        v1: GMVec2D = GMVec2D(x1, y1)
        v2: GMVec2D = GMVec2D(x2, y2)
        return cls(name, v1, v2, obj)

    @override
    def draw(self) -> None:
        pass

    @override
    def update(self, dt: float) -> None:
        pass

    @override
    def send_message(self, msg: Any) -> Any:
        """
        Process messages send to this line.

        :param msg: The actual message.
        """

        match msg:
            case _:
                # Delegate all other messages to the base class:
                super().send_message(msg)

    def set_spacing(self, spacing: float):
        pass

    def set_count(self, count: int):
        pass

