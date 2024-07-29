# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override

from green_moon_2d.gm_object import GMObject
from green_moon_2d.gm_font import GMFont
from green_moon_2d.gm_math import GMVec2D, GMAlignment
import green_moon_2d.gm_engine as gme


class GMText(GMObject):
    def __init__(
            self, name: str, text: str, pos: tuple[float, float] | GMVec2D,
            font: GMFont, alignment: GMAlignment = GMAlignment.TOP_LEFT):
        """
        :param name: The name of the text object.
        :param text: The actual text.
        :param pos: The text position.
        :param font: The font to use when drawing the text.
        """

        super().__init__(name)

        self.text: str = text
        self.font: GMFont = font
        self.horizontal: bool = True
        self.alignment: GMAlignment = alignment

        match pos:
            case (x, y):
                self.pos.x = x
                self.pos.y = y
            case GMVec2D():
                self.pos = pos
            case _:
                raise ValueError(f"GMText, position argument must be a tuple of floats or GMVec2D: {pos}")

        self.reset_chars()

    @override
    def draw(self) -> None:
        """
        Draw the text.
        """

        for (x, y, i) in self.chars:
            self.font.draw_i(x, y, i)

    @override
    def send_message(self, msg: Any) -> Any:
        """
        Process messages send to this text object.

        :param msg: The actual message.
        """

        match msg:
            case ("set_text", str(text)):
                self.set_text(text)
            case ("set_pos", pos):
                self.set_pos(pos)
            case ("set_font", font):
                self.set_font(font)
            case ("set_horizontal", bool(horizontal)):
                self.set_horizontal(horizontal)
            case ("set_alignment", GMAlignment() as alignment):
                self.set_alignment(alignment)
            case "toggle_orientation":
                self.toggle_orientation()

    def reset_chars(self) -> None:
        """
        Rests the pre-calculated character position and indices.
        Is called whenever a property is set via a setter.
        """

        wx: int = self.font.texture.unit_width
        wy: int = self.font.texture.unit_height

        textwidth: float = wx * len(self.text)
        textheight: float = wy

        if not self.horizontal:
            textwidth = wx
            textheight = wy * len(self.text)

        textwidth2: float = textwidth / 2.0
        textheight2: float = textheight / 2.0

        offsetx: float = 0.0
        offsety: float = 0.0

        match self.alignment:
            case GMAlignment.TOP_LEFT:
                pass
            case GMAlignment.TOP_CENTER:
                offsetx = textwidth2
            case GMAlignment.TOP_RIGHT:
                offsetx = textwidth
            case GMAlignment.MID_LEFT:
                offsety = textheight2
            case GMAlignment.MID_CENTER:
                offsetx = textwidth2
                offsety = textheight2
            case GMAlignment.MID_RIGHT:
                offsetx = textwidth
                offsety = textheight2
            case GMAlignment.BTM_LEFT:
                offsety = textheight
            case GMAlignment.BTM_CENTER:
                offsetx = textwidth2
                offsety = textheight
            case GMAlignment.BTM_RIGHT:
                offsetx = textwidth
                offsety = textheight
            case _:
                raise ValueError(f"GMText, unknown alignment: {self.alignment}")

        x: float = self.pos.x - offsetx
        y: float = self.pos.y - offsety

        self.chars = []
        for c in self.text:
            i: int = self.font.mapping[c]
            self.chars.append((x, y, i))

            if self.horizontal:
                x = x + wx
            else:
                y = y + wy

    def set_text(self, text: str) -> None:
        """
        Changes the text.

        :param text: The new text.
        """

        self.text = text
        self.reset_chars()

    def set_pos(self, pos: tuple[float, float] | GMVec2D) -> None:
        """
        Sets the text position.

        :param pos: The new position, must be a tuple of floats or GMVec2D.
        """

        match pos:
            case (x, y):
                self.pos.x = x
                self.pos.y = y
            case GMVec2D():
                self.pos = pos
            case _:
                raise ValueError(f"GMText, position must be a tuple of floats or GMVec2D: {pos}")

        self.reset_chars()

    def set_font(self, font: str | GMFont) -> None:
        """
        Sets the font for this text.

        :param font: Can be a GMFont or the font name as string.
        """

        match font:
            case GMFont():
                self.font = font
            case str():
                self.font = gme.GMGlobalResources.get_font(font)
            case _:
                raise ValueError(f"GMText, font must be a GMFont or a font name (str): {font}")

        self.reset_chars()

    def set_horizontal(self, horizontal: bool) -> None:
        """
        Sets the orientation (horizontal / vertical) of the this text.

        :param horizontal: If true the orientation is horizontal else vertical.
        """

        self.horizontal = horizontal
        self.reset_chars()

    def set_alignment(self, alignment: GMAlignment) -> None:
        """
        Sets the alignment for this text.

        :param alignment: The alignment (enum GMAlignment).
        """

        self.alignment = alignment
        self.reset_chars()

    def toggle_orientation(self) -> None:
        """
        """

        self.horizontal = not self.horizontal
        self.reset_chars()

