# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest

from typing import Any, override

from green_moon_2d.gm_text import GMText, GMTextEffect1
from green_moon_2d.gm_font import GMFont
from green_moon_2d.gm_math import GMVec2D, GMAlignment
from green_moon_2d.gm_texture import GMTextureInterface

class TestTexture(GMTextureInterface):
    def __init__(self, unit_width: int, unit_height: int):
        """
        :param unit_width: The width of a single frame / cell.
        :param unit_height: The heiht of a single frame / cell.
        """

        super().__init__(unit_width, unit_height)

    @override
    def draw(self, dx: float, dy: float, index: int) -> None:
        _ = dx
        _ = dy
        _ = index

    @override
    def draw_p(self, pos: GMVec2D, index: int) -> None:
        _ = pos
        _ = index

    @override
    def draw_opt(self, dx: float, dy: float, index: int, angle: float = 0.0,
            scale: float = 1.0, flip_x: bool = False, flip_y: bool = False) -> None:
        _ = dx
        _ = dy
        _ = index
        _ = angle
        _ = scale
        _ = flip_x
        _ = flip_y

    @override
    def draw_p_opt(self, pos: GMVec2D, index: int, angle: float = 0.0,
            scale: float = 1.0, flip_x: bool = False, flip_y: bool = False) -> None:
        _ = pos
        _ = index
        _ = angle
        _ = scale
        _ = flip_x
        _ = flip_y


class TestObjectManager(unittest.TestCase):
    def setUp(self):
        self.texture = TestTexture(32, 32)
        mapping = {}
        for i, c in enumerate("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!.,-+_#"):
            mapping[c] = i
        self.font = GMFont("test", self.texture, mapping)


