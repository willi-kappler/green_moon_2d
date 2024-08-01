# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMContext class which contain all game engine relevant elements.
"""

from typing import Any

import pygame

import logging
logger = logging.getLogger(__name__)


class GMContext:
    """
    This class contains acess to the scene manager and the state of the
    game engine. It also allows you to store data that is used across
    multiple scenes.
    """

    def __init__(self):
        logger.debug("Create a new GMContext.")

        self.game_property: dict[str, Any] = {}
        self.quit_game: bool = False
        self.screen: Any = None
        self.keys_up = set()
        self.keys_down = set()

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

        logger.debug("Toggle fullscreen.")

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

        self.keys_up = set()
        self.keys_down = set()

        for event in pygame.event.get():
            if event.type == pygame.KEYDOWN:
                match event.key:
                    case pygame.K_ESCAPE:
                        self.keys_down.add("ESC")
                    case pygame.K_SPACE:
                        self.keys_down.add("SPACE")
                    case pygame.K_UP:
                        self.keys_down.add("UP")
                    case pygame.K_DOWN:
                        self.keys_down.add("DOWN")
                    case pygame.K_LEFT:
                        self.keys_down.add("LEFT")
                    case pygame.K_RIGHT:
                        self.keys_down.add("RIGHT")
                    case pygame.K_0:
                        self.keys_down.add("0")
                    case pygame.K_1:
                        self.keys_down.add("1")
                    case pygame.K_2:
                        self.keys_down.add("2")
                    case pygame.K_3:
                        self.keys_down.add("3")
                    case pygame.K_4:
                        self.keys_down.add("4")
                    case pygame.K_5:
                        self.keys_down.add("5")
                    case pygame.K_6:
                        self.keys_down.add("6")
                    case pygame.K_7:
                        self.keys_down.add("7")
                    case pygame.K_8:
                        self.keys_down.add("8")
                    case pygame.K_9:
                        self.keys_down.add("9")
                    case pygame.K_a:
                        self.keys_down.add("A")
                    case pygame.K_b:
                        self.keys_down.add("B")
                    case pygame.K_c:
                        self.keys_down.add("C")
                    case pygame.K_d:
                        self.keys_down.add("D")
                    case pygame.K_e:
                        self.keys_down.add("E")
                    case pygame.K_f:
                        self.keys_down.add("F")
                    case pygame.K_g:
                        self.keys_down.add("G")
                    case pygame.K_h:
                        self.keys_down.add("H")
            elif event.type == pygame.KEYUP:
                match event.key:
                    case pygame.K_ESCAPE:
                        self.keys_up.add("ESC")
                    case pygame.K_SPACE:
                        self.keys_up.add("SPACE")
                    case pygame.K_UP:
                        self.keys_up.add("UP")
                    case pygame.K_DOWN:
                        self.keys_up.add("DOWN")
                    case pygame.K_LEFT:
                        self.keys_up.add("LEFT")
                    case pygame.K_RIGHT:
                        self.keys_up.add("RIGHT")
                    case pygame.K_0:
                        self.keys_up.add("0")
                    case pygame.K_1:
                        self.keys_up.add("1")
                    case pygame.K_2:
                        self.keys_up.add("2")
                    case pygame.K_3:
                        self.keys_up.add("3")
                    case pygame.K_4:
                        self.keys_up.add("4")
                    case pygame.K_5:
                        self.keys_up.add("5")
                    case pygame.K_6:
                        self.keys_up.add("6")
                    case pygame.K_7:
                        self.keys_up.add("7")
                    case pygame.K_8:
                        self.keys_up.add("8")
                    case pygame.K_9:
                        self.keys_up.add("9")
                    case pygame.K_a:
                        self.keys_up.add("A")
                    case pygame.K_b:
                        self.keys_up.add("B")
                    case pygame.K_c:
                        self.keys_up.add("C")
                    case pygame.K_d:
                        self.keys_up.add("D")
                    case pygame.K_e:
                        self.keys_up.add("E")
                    case pygame.K_f:
                        self.keys_up.add("F")
                    case pygame.K_g:
                        self.keys_up.add("G")
                    case pygame.K_h:
                        self.keys_up.add("H")


