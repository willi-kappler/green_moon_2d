# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override

import green_moon_2d.gm_math as gmm
from green_moon_2d.gm_message import GMMessage
from green_moon_2d.gm_object import GMObject, GMObjectManager
from green_moon_2d.gm_interpolation import GMInterpolate
from green_moon_2d.gm_timer import GMTimer

import logging
logger = logging.getLogger(__name__)

class GMMUIgnore(GMObject):
    def __init__(self, name: str):
        super().__init__(name)

    @override
    def send_message(self, msg: GMMessage) -> Any:
        # Just ignore all messages
        pass


class GMMUMap(GMObject):
    def __init__(self, name: str, func, om: GMObjectManager):
        super().__init__(name)
        self.func = func
        self.om: GMObjectManager = om
        self.user_message: GMMessage = GMMessage.empty()

    @override
    def send_message(self, msg: GMMessage) -> Any:

        child: str = msg.next_child()

        match child:
            case "base":
                return super().send_message(msg)
            case "msg":
                return self.user_message.send_message(msg)
            case "map_self":
                if msg.command == "set_func":
                    self.func = msg.value
                else:
                    raise ValueError(f"Unknown message: {msg}, {child=}")
            case _:
                self.message_received = True
                # Apply function to message value:
                self.user_message.value = self.func(msg.value)
                # Copy rest of message:
                self.user_message.command = msg.command
                # Restore child hirarchy:
                msg.insert_child(child)
                self.user_message.children = msg.children
                return self.om.send_message(self.user_message)

class GMMUTimer(GMObject):
    def __init__(self, name: str, delay: int, repeat: bool = False):
        super().__init__(name)
        self.timer: GMTimer = GMTimer(delay)
        self.user_message: GMMessage = GMMessage.empty()
        self.repeat: bool = repeat

    @override
    def update(self, dt: float, om: GMObjectManager) -> None:
        if self.timer.finished():
            if self.repeat:
                self.timer.restart()
            else:
                self.timer.active = False
            om.send_message(self.user_message)

    @override
    def send_message(self, msg: GMMessage) -> Any:
        pass



