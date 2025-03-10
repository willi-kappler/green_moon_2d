# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override

import green_moon_2d.gm_math as gmm
from green_moon_2d.gm_message import GMMessage
from green_moon_2d.gm_object import GMObject, GMObjectManager
from green_moon_2d.gm_interpolation import GMInterpolate

import logging
logger = logging.getLogger(__name__)

class GMMVCircle(GMObject):
    def __init__(self, name: str, pos: tuple[float, float] | gmm.GMVec2D,
            radius: float, speed: float):
        """
        :param name: The name of the movement. It must be unique.
        :param pos: The position of the circular movement on the screen.
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
        """
        Update circular movement.
        :param dt: time that has past since last frame.
        :param om: the object manager this object belongs to.
        """

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

        child: str = msg.next_child()

        match child:
            case "base":
                return super().send_message(msg)
            case "msg":
                return self.user_message.send_message(msg)
            case _:
                command: str = msg.command
                value = msg.value
                msg2 = (command, value)

                match msg2:
                    case ("set_radius", float(radius)):
                        self.radius = radius
                    case ("get_radius", _):
                        return self.radius
                    case ("set_speed", float(speed)):
                        self.speed = speed
                    case ("get_speed", _):
                        return self.speed
                    case _:
                        raise ValueError(f"Unknown message: {msg}, {child=}")

class GMMLine(GMObject):
    def __init__(self, name: str, pos1: tuple[float, float] | gmm.GMVec2D,
            pos2: tuple[float, float] | gmm.GMVec2D, speed: float):
        """
        :param name: The name of the movement. It must be unique.
        :param pos1: The first position of the line movement on the screen.
        :param pos2: The second position of the line movement on the screen.
        :param speed: The angular speed in deg.
        """

        super().__init__(name)
        self.interpolation = GMInterpolate(name, pos1, pos2, speed)
        self.user_message: GMMessage = GMMessage.empty()

    @override
    def update(self, dt: float, om: GMObjectManager) -> None:
        """
        Update line movement.
        :param dt: time that has past since last frame.
        :param om: the object manager this object belongs to.
        """

        self.interpolation.update(dt, om)
        self.user_message.value = self.interpolation.current_value
        om.send_message(self.user_message)

    @override
    def send_message(self, msg: GMMessage) -> Any:
        """
        Process messages send to this line movement.

        :param msg: The actual message.
        """

        child: str = msg.next_child()

        match child:
            case "base":
                return super().send_message(msg)
            case "msg":
                return self.user_message.send_message(msg)
            case "inter":
                return self.interpolation.send_message(msg)
            case _:
                raise ValueError(f"Unknown message: {msg}, {child=}")


