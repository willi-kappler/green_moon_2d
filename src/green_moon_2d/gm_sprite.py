# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override

from green_moon_2d.gm_math import GMVec2D
from green_moon_2d.gm_object import GMObject, GMObjectManager
from green_moon_2d.gm_animation import GMAnimation
from green_moon_2d.gm_texture import GMTextureInterface


class GMSprite(GMObject):
    def __init__(self, pos: tuple[float, float] | GMVec2D, name: str,
        animation: GMAnimation, texture: GMTextureInterface):
        """
        :param pos: The position of the sprite on the screen.
        :param name: The name of the sprite. It must be unique.
        :param animation: The animation for this sprite.
        :param texture: The texture / sprite sheet for this sprite.
        """

        super().__init__(name)
        self.set_pos(pos)

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
    def update(self, dt: float, om: GMObjectManager) -> None:
        self.animation.update()

    @override
    def send_message(self, msg: Any) -> Any:
        """
        Process messages send to this sprite.

        :param msg: The actual message.
        """

        match msg:
            case ("set_animation", GMAnimation() as animation):
                self.animation = animation
            case ("set_texture", GMTextureInterface() as texture):
                self.texture = texture
            case ("set_angle", float(angle)):
                self.angle = angle
            case ("set_scale", float(scale)):
                self.scale = scale
            case ("set_flip_x", bool(flip_x)):
                self.flip_x = flip_x
            case "toggle_flip_x":
                self.flip_x = not self.flip_x
            case ("set_flip_y", bool(flip_y)):
                self.flip_y = flip_y
            case "toggle_flip_y":
                self.flip_y = not self.flip_y
            case _:
                # Delegate all other messages to the base class:
                super().send_message(msg)





