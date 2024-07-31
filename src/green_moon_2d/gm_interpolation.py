# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override

from green_moon_2d.gm_object import GMObject
from green_moon_2d.gm_math import GMRepetition


def gm_curve_linear(x: float) -> float:
    return x


class GMInterpolate(GMObject):
    def __init__(self, name: str, start, end, speed: float, current_step: float):
        super().__init__(name)

        self.start = start
        self.end = end
        self.diff = end - start
        self.speed: float = speed
        self.current_step: float = current_step
        self.current_value = start + (self.diff * current_step)
        self.repetition = GMRepetition.FORWARD
        self.curve = gm_curve_linear

    def calculate_diff(self) -> None:
        self.diff = self.end - self.start

    def calculate_value(self) -> None:
        curve_value = self.curve(self.current_step)
        if curve_value < 0.0:
            curve_value = 0.0
        elif curve_value > 1.0:
            curve_value = 1.0

        self.current_value = self.start + (self.diff * curve_value)

    def reset(self):
        self.current_step = 0.0
        self.current_value = self.start

    @override
    def update(self) -> None:
        raise NotImplementedError
        # TODO: implement update

    @override
    def send_message(self, msg: Any) -> Any:
        raise NotImplementedError
        # TODO: implement send_message

    def is_finished(self) -> bool:
        raise NotImplementedError
        # TODO: implement is_finished





