# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from green_moon_2d.gm_texture import GMTexture


class GMFont:
    def __init__(self, texture: GMTexture, mapping: dict[str, int]):
        self.texture = texture
        self.mapping = mapping

    def draw(self, x: float, y: float, c: str):
        """
        Draws one character on the given position using the current texture and mapping.

        :param x: The x position of the character.
        :param y: The y position of the character.
        :param c: The character itself.
        """

        i = self.mapping[c]
        self.texture.draw(x, y, i)

