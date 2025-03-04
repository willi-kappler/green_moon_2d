# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override
import math
import random

from green_moon_2d.gm_object import GMObject, GMObjectManager
from green_moon_2d.gm_font import GMFont
from green_moon_2d.gm_math import GMVec2D, GMAlignment
from green_moon_2d.gm_timer import GMTimer
import green_moon_2d.gm_engine as gme

import logging
logger = logging.getLogger(__name__)


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

        logger.debug(f"Create a new text with name: {name} and text: {text}.")

        self.text: str = text
        self.font: GMFont = font
        self.horizontal: bool = True
        self.alignment: GMAlignment = alignment

        match pos:
            case (float(x), float(y)):
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
                self.reset_chars()
            case ("set_font", font):
                self.set_font(font)
            case ("set_horizontal", bool(horizontal)):
                self.set_horizontal(horizontal)
            case ("set_alignment", GMAlignment() as alignment):
                self.set_alignment(alignment)
            case "toggle_orientation":
                self.toggle_orientation()
            case _:
                # Delegate all other messages to the base class:
                super().send_message(msg)

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

    def set_font(self, font: str | GMFont) -> None:
        """
        Sets the font for this text.

        :param font: Can be a GMFont or the font name as string.
        """

        logger.debug(f"Change the font to: {font}.")

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

        logger.debug(f"Change horizontal orientation: {horizontal}.")

        self.horizontal = horizontal
        self.reset_chars()

    def set_alignment(self, alignment: GMAlignment) -> None:
        """
        Sets the alignment for this text.

        :param alignment: The alignment (enum GMAlignment).
        """

        logger.debug(f"Set alignment: {alignment}.")

        self.alignment = alignment
        self.reset_chars()

    def toggle_orientation(self) -> None:
        """
        Toggle the orientation from horizontal to vertical and back again.
        """

        logger.debug("Toggle orientation.")

        self.horizontal = not self.horizontal
        self.reset_chars()


class GMTextEffect1(GMText):
    """
    This class contains four text effects: sine wave, rotate, scale and jitter.
    Sine wave moves the characters along a sine curve.
    Rotate will rotate each character individually.
    Scale will scale each character individually up and down.
    Jitter will add some random jitter to each character position.
    """

    def __init__(
            self, name: str, text: str, pos: tuple[float, float] | GMVec2D,
            font: GMFont, alignment: GMAlignment = GMAlignment.TOP_LEFT):
        """
        :param name: The name of the text object.
        :param text: The actual text.
        :param pos: The text position.
        :param font: The font to use when drawing the text.
        """

        super().__init__(name, text, pos, font, alignment)

        logger.debug("Create a new GMTextEffect1.")

        self.effect_sine: bool = False
        self.effect_rotate: bool = False
        self.effect_scale: bool = False
        self.effect_jitter: bool = False

        # Sine effect settings:
        self.sine_amplitude: float = 20.0
        self.sine_frequency: float = 0.006
        self.sine_offset: float = 0.4
        self.sine_dt: float = 0.0

        # Rotate effect settings:
        self.rotate_frequency: float = 0.08
        self.rotate_offset: float = 2.0
        self.rotate_dt: float = 0.0

        self.scale_amplitude: float = 0.4
        self.scale_frequency: float = 0.006
        self.scale_offset: float = 0.1
        self.scale_dt: float = 0.0

        self.jitter_radius: float = 5.0
        self.jitter_timer: GMTimer = GMTimer(100)

        self.chars_copy = self.chars.copy()

    @override
    def update(self, dt: float, om: GMObjectManager) -> None:
        """
        Updates all the text effects.
        """

        self.chars = self.chars_copy.copy()
        offset: float = 0.0

        if self.effect_sine:
            self.sine_dt = self.sine_dt + (self.sine_frequency * dt)
            if self.sine_dt > math.tau:
                self.sine_dt = self.sine_dt - math.tau

            if self.horizontal:
                for j, (x, y, i) in enumerate(self.chars):
                    ny = y + (self.sine_amplitude * math.sin(self.sine_dt + offset))
                    offset = offset + self.sine_offset
                    self.chars[j] = (x, ny, i)
            else:
                for j, (x, y, i) in enumerate(self.chars):
                    nx = x + (self.sine_amplitude * math.sin(self.sine_dt + offset))
                    offset = offset + self.sine_offset
                    self.chars[j] = (nx, y, i)

        if self.effect_rotate:
            self.rotate_dt = self.rotate_dt + (self.rotate_frequency * dt)
            if self.rotate_dt > 360.0:
                self.rotate_dt = self.rotate_dt - 360.0
            elif self.rotate_dt < -360.0:
                self.rotate_dt = self.rotate_dt + 360.0

            offset = 0.0

            for i in range(len(self.chars)):
                self.rotate_angles[i] = self.rotate_dt + offset
                offset = offset + self.rotate_offset

        if self.effect_scale:
            self.scale_dt = self.scale_dt + (self.scale_frequency * dt)
            if self.scale_dt > math.tau:
                self.scale_dt = self.scale_dt - math.tau

            offset = 0.0

            for i in range(len(self.chars)):
                self.scale_values[i] = 1.0 + (self.scale_amplitude *
                    math.sin(self.scale_dt + offset))
                offset = offset + self.scale_offset

        if self.effect_jitter:
            if self.jitter_timer.finished():
                for i in range(len(self.chars)):
                    jx = (random.random() - 0.5) * self.jitter_radius * 2.0
                    jy = (random.random() - 0.5) * self.jitter_radius * 2.0
                    self.jitter_positions[i] = (jx, jy)
                self.jitter_timer.restart()

    @override
    def draw(self) -> None:
        """
        Draw the text.
        """

        for i in range(len(self.chars)):
            (x, y, n) = self.chars[i]
            angle = self.rotate_angles[i]
            scale = self.scale_values[i]
            (jx, jy) = self.jitter_positions[i]
            self.font.texture.draw_opt(x + jx, y + jy, n, angle, scale)

    @override
    def reset_chars(self) -> None:
        """
        Re-calculate all the character positions and indices.
        """

        super().reset_chars()

        self.chars_copy = self.chars.copy()

        self.rotate_angles: list[float] = []
        self.scale_values: list[float] = []
        self.jitter_positions: list[tuple[float, float]] = []

        for _ in range(len(self.chars)):
            self.rotate_angles.append(0.0)
            self.scale_values.append(1.0)
            self.jitter_positions.append((0.0, 0.0))

    def send_message(self, msg: Any) -> Any:
        match msg:
            case ("effect_sine", bool(active)):
                self.effect_sine = active
            case ("effect_rotate", bool(active)):
                self.effect_rotate = active
            case ("effect_scale", bool(active)):
                self.effect_scale = active
            case ("effect_jitter", bool(active)):
                self.effect_jitter = active
            case "toggle_sine":
                self.toggle_sine()
            case "toggle_rotate":
                self.toggle_rotate()
            case "toggle_scale":
                self.toggle_scale()
            case "toggle_jitter":
                self.toggle_jitter()
            case ("set_sine_amplitude", float(amplitude)):
                self.sine_amplitude = amplitude
            case ("set_sine_frequency", float(frequency)):
                self.sine_frequency = frequency
            case ("set_sine_offset", float(offset)):
                self.sine_offset = offset
            case ("set_rotate_frequency", float(frequency)):
                self.rotate_frequency = frequency
            case ("set_rotate_offset", float(offset)):
                self.rotate_offset = offset
            case ("set_scale_amplitude", float(amplitude)):
                self.scale_amplitude = amplitude
            case ("set_scale_frequency", float(frequency)):
                self.frequency = frequency
            case ("set_scale_offset", float(offset)):
                self.scale_offset = offset
            case ("set_jitter_radius", float(radius)):
                self.jitter_radius = radius
            case ("set_jitter_timer", int(duration)):
                self.jitter_timer.set_duration(duration)
            case _:
                # Delegate all other messages to the base class:
                super().send_message(msg)

    def toggle_sine(self) -> None:
        """
        Turn the sine effect on or off.
        """

        logger.debug("Toggle sine.")

        self.effect_sine = not self.effect_sine

    def toggle_rotate(self) -> None:
        """
        Turn the rotate effect on or off.
        """

        logger.debug("Toggle rotate.")

        self.effect_rotate = not self.effect_rotate

        if not self.effect_rotate:
            for i in range(len(self.chars)):
                self.rotate_angles[i] = 0.0

    def toggle_scale(self) -> None:
        """
        Turn the scale effect on or off.
        """

        logger.debug("Toggle scale.")

        self.effect_scale = not self.effect_scale

        if not self.effect_scale:
            for i in range(len(self.chars)):
                self.scale_values[i] = 1.0

    def toggle_jitter(self) -> None:
        """
        Turn the jitter effect on or off.
        """

        logger.debug("Toggle jitter.")

        self.effect_jitter = not self.effect_jitter

        if not self.effect_jitter:
            for i in range(len(self.chars)):
                self.jitter_positions[i] = (0.0, 0.0)



