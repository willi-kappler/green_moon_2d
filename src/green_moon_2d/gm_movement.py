# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override

import green_moon_2d.gm_math as gmm
from green_moon_2d.gm_message import GMMessage
from green_moon_2d.gm_object import GMObject, GMObjectManager

import logging
logger = logging.getLogger(__name__)

class GMMVCircle(GMObject):
    def __init__(self, pos: tuple[float, float] | gmm.GMVec2D,
            name: str, radius: float, speed: float):
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
        # Angle in degree
        self.angle: float = 0.0

        self.user_message: GMMessage = GMMessage.empty()

    @override
    def update(self, dt: float, om: GMObjectManager) -> None:
        self.angle += self.speed * dt
        t: tuple[float, float] = gmm.gm_orbit_circle2(self.pos, self.radius, self.angle)
        self.user_message.value = t
        om.send_message(self.user_message)

    @override
    def send_message(self, msg: GMMessage) -> Any:
        """
        Process messages send to this circular movement.

        :param msg: The actual message.
        """

        match msg:
            case ():
                pass
