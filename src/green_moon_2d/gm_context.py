# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMContext class which is passed to all the update()
and draw() methods.
"""

from typing import Any

import sdl2
import sdl2.ext

from green_moon_2d.gm_configuration import GMConfiguration
from green_moon_2d.gm_scene import GMSceneManager


class GMContext:
    """
    This class contains acess to the scene manager and the state of the
    game engine. It also allows you to store data that is used across
    multiple scenes.
    """

    def __init__(self):
        self.game_property: dict[str, Any] = {}
        self.config = GMConfiguration()
        self.scene_manager: GMSceneManager = GMSceneManager()
        self.quit_game: bool = False
        self.renderer: Any = None

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

    def load_config(self, file_name: str):
        """
        Load the configuration from the given file name.

        :param file_name: The name of the configuration file.
        """
        self.config.load_config(file_name)

    def load_resources(self):
        """
        Loads the resources from the configuration file.
        """

        raise NotImplementedError
        # TODO: Load resources from JSON file.

    def set_fullscreen(self):
        """
        Sets the window to full screen or windowed mode, depenting on the setting in the config option.
        """

        if self.config.fullscreen:
            sdl2.SDL_SetWindowFullscreen(sdl2.SDL_WINDOW_FULLSCREEN)
        else:
            sdl2.SDL_SetWindowFullscreen(0)

    def toggle_fullscreen(self):
        """
        Toggle between fullscreen and windowed mode.
        """

        self.config.fullscreen = not self.config.fullscreen
        self.set_fullscreen()

    def update(self):
        """
        Updates the current scene in the scene manager.
        """

        self.scene_manager.update(self)

    def draw(self):
        """
        Draws the current scene in the scene manager.
        """

        self.scene_manager.draw(self)



