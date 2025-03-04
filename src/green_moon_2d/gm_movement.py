# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override

from green_moon_2d.gm_math import GMVec2D
from green_moon_2d.gm_object import GMObject, GMObjectManager

import logging
logger = logging.getLogger(__name__)

class GMMVCircle(GMObject):
    def __init__(self, pos: tuple[float, float] | GMVec2D, name: str, radius: float, speed: float):
        """
        :param pos: The position of the circular movement on the screen.
        :param name: The name of the movement. It must be unique.
        :param radius: The radius of the circle.
        :param speed: The angular speed in deg.
        """

        super().__init__(name)
        self.set_pos(pos)

        self.radius: float = radius
        self.speed: float = speed

        self.target: str = ""
        self.target_part: str = ""
        self.command: str = ""

    def set_target(self, target: str):
        self.target = target

    def set_target_part(self, target_part: str):
        self.target_part = target_part

    def set_command(self, command: str):
        self.command = command

    @override
    def update(self, dt: float, om: GMObjectManager) -> None:
        pass

    @override
    def send_message(self, msg: Any) -> Any:
        """
        Process messages send to this circular movement.

        :param msg: The actual message.
        """

        match msg:
            case ():
                pass
