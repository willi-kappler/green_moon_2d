# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any

from green_moon_2d.gm_context import GMContext
from green_moon_2d.gm_object import GMObject


class GMSprite(GMObject):
    def __init__(self, name: str):
        super().__init__(name)

    def draw(self, context: GMContext) -> None:
        context.clear_screen()
        raise NotImplementedError

    def update(self, context: GMContext) -> None:
        context.clear_screen()
        raise NotImplementedError

    def send_message(self, msg: Any) -> Any:
        if isinstance(msg, str):
            raise NotImplementedError
