# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override
import math

from green_moon_2d.gm_object import GMObject, GMObjectManager
from green_moon_2d.gm_math import GMRepetition


def gm_curve_linear(x: float) -> float:
    return x


def gm_curve_x2_up(x: float) -> float:
    return x*x


def gm_curve_x2_down(x: float) -> float:
    return 1 - (x*x)


def gm_curve_slope_in_out(x: float) -> float:
    if x < 0.5:
        return math.pow(x, 4.0) * 8.0
    else:
        return 1.0 - (math.pow(x - 1.0, 4.0) * 8.0)


def gm_curve_sinus(x: float) -> float:
    return (math.sin((x * math.pi) - (math.pi / 2.0)) + 1.0) / 2.0


class GMInterpolate(GMObject):
    def __init__(self, name: str, start, end, speed: float = 0.1, current_step: float = 0.0):
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

    def reset(self) -> None:
        self.current_step = 0.0
        self.current_value = self.start

    @override
    def update(self, dt: float, om: GMObjectManager) -> None:
        match self.repetition:
            case GMRepetition.FIXED:
                # Nothing to do.
                pass
            case GMRepetition.FORWARD:
                if self.current_step < 1.0:
                    self.current_step = self.current_step + (self.speed * dt)
                    if self.current_step > 1.0:
                        self.current_step = 1.0
            case GMRepetition.BACKWARD:
                if self.current_step > 0.0:
                    self.current_step = self.current_step - (self.speed * dt)
                    if self.current_step < 0.0:
                        self.current_step = 0.0
            case GMRepetition.FORWARD_LOOP:
                self.current_step = self.current_step + (self.speed * dt)
                if self.current_step > 1.0:
                    self.current_step = 0.0
            case GMRepetition.BACKWARD_LOOP:
                self.current_step = self.current_step - (self.speed * dt)
                if self.current_step < 0.0:
                    self.current_step = 1.0
            case GMRepetition.PINGPONG_F:
                self.current_step = self.current_step + (self.speed * dt)
                if self.current_step > 1.0:
                    self.current_step = 1.0
                    self.repetition = GMRepetition.PINGPONG_B
            case GMRepetition.PINGPONG_B:
                self.current_step = self.current_step - (self.speed * dt)
                if self.current_step < 0.0:
                    self.current_step = 0.0
                    self.repetition = GMRepetition.PINGPONG_F

        self.calculate_value()

    @override
    def send_message(self, msg: Any) -> Any:
        match msg:
            case ("set_start", start):
                self.start = start
                self.calculate_diff()
            case ("set_end", end):
                self.end = end
                self.calculate_diff()
            case ("set_speed", float(speed)):
                self.speed = speed
            case ("set_repetition", GMRepetition() as repetition):
                self.set_repetition(repetition)
            case ("set_curve", curve):
                self.curve = curve
            case "is_finished":
                return self.is_finished()
            case _:
                super().send_message(msg)

    def is_finished(self) -> bool:
        match self.repetition:
            case GMRepetition.FIXED:
                return True
            case GMRepetition.FORWARD:
                return math.isclose(self.current_step, 1.0)
            case GMRepetition.BACKWARD:
                return math.isclose(self.current_step, 0.0)
            case _:
                return False

    def set_repetition(self, repetition: GMRepetition) -> None:
        match repetition:
            case GMRepetition.FIXED:
                self.current_step = 0.0
            case GMRepetition.FORWARD:
                self.current_step = 0.0
            case GMRepetition.BACKWARD:
                self.current_step = 1.0
            case GMRepetition.FORWARD_LOOP:
                self.current_step = 0.0
            case GMRepetition.BACKWARD_LOOP:
                self.current_step = 1.0
            case GMRepetition.PINGPONG_F:
                self.current_step = 0.0
            case GMRepetition.PINGPONG_B:
                self.current_step = 1.0

        self.repetition = repetition




