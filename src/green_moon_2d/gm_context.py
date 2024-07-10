# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMContext class which is passed to all the update()
and draw() methods.
"""

from typing import Any


class GMContext:
    """
    This class contains acess to the scene manager and the state of the
    game engine. It also allows you to store data that is used across
    multiple scenes.
    """

    def __init__(self):
        self.game_property = {}

    def set_property(self, name: str, val: Any) -> None:
        """
        Sets the given property globally for the whole program.

        :param str name: The name of the property.
        :param Any val: The value of the property.
        """
        self.game_property[name] = val

    def get_property(self, name: str) -> Any:
        """
        Returns the value of the given property.

        :param str name: The name of the property.
        :return: The value of the property.
        :rtype: Any
        :raises NameError: if the property with the given name was not found.
        """
        return self.game_property[name]

    def has_property(self, name: str) -> bool:
        """
        Returns true if the given property is available.
        Otherwise return false.

        :param str name: The name of the property.
        :return: If the property with the given name is available return True otherwise False.
        :rtype: bool
        """
        return name in self.game_property

    def clear_screen(self) -> None:
        """
        Clears the screen with the black color.
        """
        raise NotImplementedError
