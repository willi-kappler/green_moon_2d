# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import pygame

import green_moon_2d.gm_engine as gme

import logging
logger = logging.getLogger(__name__)


class GMTexture:
    def __init__(self, unit_width: int, unit_height: int, surface: pygame.Surface):
        """
        :param unit_width: The width of a single frame / cell.
        :param unit_height: The heiht of a single frame / cell.
        :param texture: The actual graphic texture.
        """

        logger.debug(f"Create a new GMTexture with unit dimensions: {unit_width} x {unit_height}.")

        self.surface: pygame.Surface = surface
        self.unit_width: int = unit_width
        self.unit_height: int = unit_height
        self.cols: int = int(surface.get_width() / unit_width)

    def get_subsurface(self, index: int) -> pygame.Surface:
        """
        Return a subsurface from the texture given the index.

        :param index: The index of the frame / cell.
        """

        yi: int = int(index / self.cols)
        xi: int = index - (yi * self.cols)

        sx: int = xi * self.unit_width
        sy: int = yi * self.unit_height

        area = pygame.Rect(sx, sy, self.unit_width, self.unit_height)
        subsurface = self.surface.subsurface(area)

        return subsurface

    def draw(self, dx: float, dy: float, index: int):
        """
        Draw this texture on the screen given the coordinates and frame index.

        :param dx: The center x position.
        :param dy: The center y position.
        :param index: The index of the frame / cell.
        """

        dx = (dx - (self.unit_width / 2.0))
        dy = (dy - (self.unit_height / 2.0))

        subsurface = self.get_subsurface(index)

        gme.GMGlobalContext.screen.blit(subsurface, (dx, dy))

    def draw_opt(
            self, dx: float, dy: float, index: int, angle: float = 0.0,
            scale: float = 1.0, flip_x: bool = False,
            flip_y: bool = False):
        """
        Draw this texture on the screen using several options.

        :param dx: The center x position.
        :param dy: The center y position.
        :param index: The index of the frame / cell.
        :param angle: Rotate the texture.
        :param scale: The scale of the texture.
        :param flip_x: True to flip horizontally.
        :param flip_y: True to flip vertically.
        """

        dx = (dx - (self.unit_width / 2.0))
        dy = (dy - (self.unit_height / 2.0))

        subsurface = self.get_subsurface(index)
        subsurface = pygame.transform.flip(subsurface, flip_x, flip_y)
        subsurface = pygame.transform.rotozoom(subsurface, angle, scale)

        gme.GMGlobalContext.screen.blit(subsurface, (dx, dy))

