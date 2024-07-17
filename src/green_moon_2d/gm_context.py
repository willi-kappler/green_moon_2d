# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMContext class which contain all game engine relevant elements.
"""

from typing import Any

import sdl2  # type: ignore
import sdl2.ext  # type: ignore

from green_moon_2d.gm_configuration import GMConfiguration
from green_moon_2d.gm_interfaces import GMSceneInterface
from green_moon_2d.gm_messages import GMSceneMessage


class GMContext:
    """
    This class contains acess to the scene manager and the state of the
    game engine. It also allows you to store data that is used across
    multiple scenes.
    """

    def __init__(self):
        self.game_property: dict[str, Any] = {}
        self.config = GMConfiguration()
        self.quit_game: bool = False
        self.renderer: Any = None
        self.scene_messages: list[GMSceneMessage] = []

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

    def set_screen_mode(self):
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
        self.set_screen_mode()

    def clear(self):
        """
        Clears the screen with black color.
        """

        self.renderer.clear()

    def present(self):
        """
        Updates the screen with all changes (batched mode).
        """

        self.renderer.present()

    def add_scene(self, scene: GMSceneInterface):
        """
        Creates a message to add a new scene to the scene manager.

        :param scene: The new scene to be added.
        """

        self.scene_messages.append(GMSceneMessage("add", scene=scene))

    def delete_scene(self, name: str):
        """
        Create a message to elete the scene with the given name.

        :param name: The name of the scene to be deleted.
        """

        self.scene_messages.append(GMSceneMessage("delete", scene_name=name))

    def change_scene(self, name: str):
        """
        Changes the current scene to the named scene.

        :param name: The name of the scene to change to.
        """

        self.scene_messages.append(GMSceneMessage("change", scene_name=name))

    def push_and_change_scene(self, name: str):
        """
        Creates a message to push the current scene onto the stack and change to the given scene.

        :param name: The name of the scene to be changed to.
        """

        self.scene_messages.append(GMSceneMessage("push", scene_name=name))

    def pop_scene(self):
        """
        Creates a message to pop the scene from the stack and switch to it.
        """

        self.scene_messages.append(GMSceneMessage("pop"))

    def update_stack_top(self):
        """
        Creates a message to
        """

        self.scene_messages.append(GMSceneMessage("update_stack_top"))

    def draw_stack_top(self):
        """
        Creates a message to
        """

        self.scene_messages.append(GMSceneMessage("draw_stack_top"))

    def update_scene(self, name: str):
        """
        Creates a message to

        :param name: The name of the scene where update should be called.
        """

        self.scene_messages.append(GMSceneMessage("update_scene", scene_name=name))

    def draw_scene(self, name: str):
        """
        Creates a message to

        :param name: The name of the scene where draw should be called.
        """

        self.scene_messages.append(GMSceneMessage("draw_scene", scene_name=name))

    def send_message(self, name: str, custom_message: Any):
        """
        Creates a message to

        :param name: The name of the scene where draw should be called.
        """

        self.scene_messages.append(GMSceneMessage("draw_scene",
            scene_name=name, custom_message=custom_message))


