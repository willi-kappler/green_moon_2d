# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMContext class which contain all game engine relevant elements.
"""

from typing import Any

import pygame


class GMContext:
    """
    This class contains acess to the scene manager and the state of the
    game engine. It also allows you to store data that is used across
    multiple scenes.
    """

    def __init__(self):
        self.game_property: dict[str, Any] = {}
        self.quit_game: bool = False
        self.screen: Any = None
        self.dt: int = 0
        self.keys: dict[str, bool] = {
            "ESC": False,
            "UP": False,
            "DOWN": False,
            "LEFT": False,
            "RIGHT": False,
            "1": False,
            "2": False,
            "3": False,
            "4": False,
            "5": False,
            "6": False,
            "7": False,
            "8": False,
            "9": False,
            "0": False,
        }

    def set_property(self, name: str, val: Any) -> None:
        """
        Sets the given property globally for the whole program.

        :param name: The name of the property.
        :param val: The value of the property.
        """

        self.game_property[name] = val

    def get_property(self, name: str) -> Any:
        """
        Returns the value of the given property.

        :param name: The name of the property.
        :return: The value of the property.
        :rtype: Any
        :raises NameError: if the property with the given name was not found.
        """

        return self.game_property[name]

    def has_property(self, name: str) -> bool:
        """
        Returns true if the given property is available.
        Otherwise return false.

        :param name: The name of the property.
        :return: If the property with the given name is available return True otherwise False.
        :rtype: bool
        """

        return name in self.game_property

    def toggle_fullscreen(self):
        """
        Toggle between fullscreen and windowed mode.
        """

        pygame.display.toggle_fullscreen()

    def clear_screen(self):
        """
        Clears the screen with black color.
        """

        self.screen.fill("black")

    def update_input(self):
        """
        Checks for user input
        """

        for event in pygame.event.get():
            if event.type == pygame.KEYDOWN:
                match event.key:
                    case pygame.K_ESCAPE:
                        self.keys["ESC"] = True
            elif event.type == pygame.KEYUP:
                match event.key:
                    case pygame.K_ESCAPE:
                        self.keys["ESC"] = False




