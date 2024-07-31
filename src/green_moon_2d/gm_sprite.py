# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override

from green_moon_2d.gm_object import GMObject
from green_moon_2d.gm_animation import GMAnimation
from green_moon_2d.gm_texture import GMTextureInterface

import logging
logger = logging.getLogger(__name__)


class GMSprite(GMObject):
    def __init__(self, name: str, animation: GMAnimation, texture: GMTextureInterface):
        """
        :param name: The name of the sprite. It must be unique.
        :param animation: The animation for this sprite.
        :param texture: The texture / sprite sheet for this sprite.
        """

        super().__init__(name)

        self.animation: GMAnimation = animation
        self.texture: GMTextureInterface = texture
        self.angle: float = 0.0
        self.scale: float = 1.0
        self.flip_x: bool = False
        self.flip_y: bool = False

    @override
    def draw(self) -> None:
        index = self.animation.get_frame_index()
        self.texture.draw_p_opt(self.pos, index, self.angle, self.scale, self.flip_x, self.flip_y)

    @override
    def update(self) -> None:
        self.animation.update()

    @override
    def send_message(self, msg: Any) -> Any:
        """
        Process messages send to this text object.

        :param msg: The actual message.
        """

        match msg:
            case ("set_animation", GMAnimation() as animation):
                self.animation = animation





