# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override

from green_moon_2d.gm_context import GMContext
from green_moon_2d.gm_object import GMObject


class GMSprite(GMObject):
    def __init__(self, name: str):
        """
        :param name: The name of the sprite. It must be unique.
        """
        super().__init__(name)

    @override
    def draw(self, context: GMContext) -> None:
        raise NotImplementedError

    @override
    def update(self, context: GMContext) -> None:
        raise NotImplementedError

    @override
    def send_message(self, msg: Any) -> Any:
        if isinstance(msg, str):
            raise NotImplementedError
