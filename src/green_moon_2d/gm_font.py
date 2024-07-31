# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from green_moon_2d.gm_texture import GMTextureInterface

import logging
logger = logging.getLogger(__name__)


class GMFont:
    def __init__(self, name: str, texture: GMTextureInterface, mapping: dict[str, int]):
        """
        :param name: The name of this font.
        :param mapping: The character to index mapping for this font.
        """

        logger.debug(f"Create a new GMFont, name: {name}, mapping: {mapping}")

        self.name: str = name
        self.texture = texture
        self.mapping = mapping

    def __repr__(self) -> str:
        return f"GMFONT({self.name})"

    def draw_c(self, x: float, y: float, c: str):
        """
        Draws one character on the given position using the current texture and mapping.

        :param x: The x position of the character.
        :param y: The y position of the character.
        :param c: The character itself.
        """

        i = self.mapping[c]
        self.texture.draw(x, y, i)

    def draw_i(self, x: float, y: float, index: int):
        """
        Draws one character given by the index at the given position.

        :param x: The x position of the character.
        :param y: The y position of the character.
        :param index: The character index according to the mapping.
        """

        self.texture.draw(x, y, index)

