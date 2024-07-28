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
            "SPACE": False,
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
            "A": False,
            "B": False,
            "C": False,
            "D": False,
            "E": False,
            "F": False,
            "G": False,
            "H": False,
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
                    case pygame.K_SPACE:
                        self.keys["SPACE"] = True
                    case pygame.K_UP:
                        self.keys["UP"] = True
                    case pygame.K_DOWN:
                        self.keys["DOWN"] = True
                    case pygame.K_LEFT:
                        self.keys["LEFT"] = True
                    case pygame.K_RIGHT:
                        self.keys["RIGHT"] = True
                    case pygame.K_0:
                        self.keys["0"] = True
                    case pygame.K_1:
                        self.keys["1"] = True
                    case pygame.K_2:
                        self.keys["2"] = True
                    case pygame.K_3:
                        self.keys["3"] = True
                    case pygame.K_4:
                        self.keys["4"] = True
                    case pygame.K_5:
                        self.keys["5"] = True
                    case pygame.K_6:
                        self.keys["6"] = True
                    case pygame.K_7:
                        self.keys["7"] = True
                    case pygame.K_8:
                        self.keys["8"] = True
                    case pygame.K_9:
                        self.keys["9"] = True
                    case pygame.K_a:
                        self.keys["A"] = True
                    case pygame.K_b:
                        self.keys["B"] = True
                    case pygame.K_c:
                        self.keys["C"] = True
                    case pygame.K_d:
                        self.keys["D"] = True
                    case pygame.K_e:
                        self.keys["E"] = True
                    case pygame.K_f:
                        self.keys["F"] = True
                    case pygame.K_g:
                        self.keys["G"] = True
                    case pygame.K_h:
                        self.keys["H"] = True
            elif event.type == pygame.KEYUP:
                match event.key:
                    case pygame.K_ESCAPE:
                        self.keys["ESC"] = False
                    case pygame.K_SPACE:
                        self.keys["SPACE"] = False
                    case pygame.K_UP:
                        self.keys["UP"] = False
                    case pygame.K_DOWN:
                        self.keys["DOWN"] = False
                    case pygame.K_LEFT:
                        self.keys["LEFT"] = False
                    case pygame.K_RIGHT:
                        self.keys["RIGHT"] = False
                    case pygame.K_0:
                        self.keys["0"] = False
                    case pygame.K_1:
                        self.keys["1"] = False
                    case pygame.K_2:
                        self.keys["2"] = False
                    case pygame.K_3:
                        self.keys["3"] = False
                    case pygame.K_4:
                        self.keys["4"] = False
                    case pygame.K_5:
                        self.keys["5"] = False
                    case pygame.K_6:
                        self.keys["6"] = False
                    case pygame.K_7:
                        self.keys["7"] = False
                    case pygame.K_8:
                        self.keys["8"] = False
                    case pygame.K_9:
                        self.keys["9"] = False
                    case pygame.K_a:
                        self.keys["A"] = False
                    case pygame.K_b:
                        self.keys["B"] = False
                    case pygame.K_c:
                        self.keys["C"] = False
                    case pygame.K_d:
                        self.keys["D"] = False
                    case pygame.K_e:
                        self.keys["E"] = False
                    case pygame.K_f:
                        self.keys["F"] = False
                    case pygame.K_g:
                        self.keys["G"] = False
                    case pygame.K_h:
                        self.keys["H"] = False




