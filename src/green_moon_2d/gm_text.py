# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import override

import green_moon_2d.gm_object as gmobj
import green_moon_2d.gm_font as gmfnt
from green_moon_2d.gm_math import GMVec2D


class GMText(gmobj.GMObject):
    def __init__(self, name: str, text: str, pos: tuple[float, float] | GMVec2D, font: gmfnt.GMFont):
        """
        :param name: The name of the text object.
        :param text: The actual text.
        :param pos: The text position.
        :param font: The font to use when drawing the text.
        """

        super().__init__(name)
        self.text: str = text
        self.font: gmfnt.GMFont = font
        self.horizontal: bool = True

        match pos:
            case (x, y):
                self.pos.x = x
                self.pos.y = y
            case GMVec2D():
                self.pos = pos

    @override
    def update(self):
        """
        """

        pass

    @override
    def draw(self):
        """
        """

        sx = self.pos.x
        sy = self.pos.y
        wx = self.font.texture.unit_width

        for c in self.text:
            self.font.draw(sx, sy, c)
            sx = sx + wx



