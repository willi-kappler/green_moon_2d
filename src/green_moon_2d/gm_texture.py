# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any

import sdl2
import sdl2.ext

from green_moon_2d.gm_context import GMContext


class GMTexture:
    def __init__(self, unit_width: int, unit_height: int, texture: Any):
        """
        :param unit_width: The width of a single frame / cell.
        :param unit_height: The heiht of a single frame / cell.
        :param texture: The actual graphic texture.
        """

        self.texture: Any = texture
        self.unit_width: int = unit_width
        self.unit_height: int = unit_height
        self.cols: int = texture.width / unit_width

    def draw(self, dx: float, dy: float, index: int, context: GMContext):
        """
        Draw this texture on the screen given the coordinates and frame index.

        :param dx: The center x position.
        :param dy: The center y position.
        :param index: The index of the frame / cell.
        :param context: The current game context.
        """

        self.draw_opt(dx, dy, index, context)

    def draw_opt(self, dx: float, dy: float, index: int, context: GMContext,
            angle: float = 0.0, scale: float = 1.0, flip_x: bool = False,
            flip_y: bool = False):
        """
        Draw this texture on the screen using several options.

        :param dx: The center x position.
        :param dy: The center y position.
        :param index: The index of the frame / cell.
        :param context: The current game context.
        :param angle: Rotate the texture.
        :param scale: The scale of the texture.
        :param flip_x: True to flip horizontally.
        :param flip_y: True to flip vertically.
        """

        yi = index / self.cols
        xi = index - (yi * self.cols)

        sx = xi * self.unit_width
        sy = yi * self.unit_height

        # TODO:Use scale for destination rectangle
        # See: https://github.com/willi-kappler/green_moon_2d/blob/refactor5/src/texture.rs

        dx = dx - (float(self.unit_width) / 2.0)
        dy = dy - (float(self.unit_height) / 2.0)

        srcrect = (sx, sy, self.unit_width, self.unit_height)
        dstrect = (dx, dy, self.unit_width * scale, self.unit_height * scale)

        flip = sdl2.SDL_FLIP_NONE

        if flip_x:
            flip = flip | sdl2.SDL_FLIP_HORIZONTAL

        if flip_y:
            flip = flip | sdl2.SDL_FLIP_VERTICAL

        context.renderer.copy(self.texture, srcrect, dstrect, angle, flip=flip)



