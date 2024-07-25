# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from green_moon_2d.gm_context import GMContext
from green_moon_2d.gm_texture import GMTexture


class GMFont:
    def __init__(self, texture: GMTexture, mapping: dict[str, int]):
        self.texture = texture
        self.mapping = mapping

    def draw(self, x: float, y: float, c: str, context: GMContext):
        i = self.mapping[c]
        self.texture.draw(x, y, i, context)

