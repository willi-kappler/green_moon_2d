# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMSceneManager and the GMScene base class.
"""

from typing import Any

import logging
logger = logging.getLogger(__name__)


class GMScene:
    """
    This is the base class for a user defined scene.
    You have to provide a scene name and imlement the update() and draw() methods accordingly.
    """

    def __init__(self, name: str):
        """
        :param name: The name of the new scene. It must be unique.
        """

        logger.debug(f"Create a new GMScene with name: {name}.")

        self.name: str = name
        self.on_stack: bool = False
        self.initialized: bool = False
        self.custom_property: dict[str, Any] = {}

    def update(self) -> None:
        """
        This method is called once per frame and is used to update the
        internal state of the scene. You have to implement this method.
        """

        pass

    def draw(self) -> None:
        """
        This method is called once per frame and is used to draw the
        whole scene. You have to implement this method.
        """

        pass

    def enter(self) -> None:
        """
        This method is called every time this scene becomes the current
        scene. You don't have to implement this mwthod.
        """

        pass

    def leave(self) -> None:
        """
        This method is called every time this scene is no longer the
        current scene. You don't have to implement this mwthod.
        """

        pass

    def send_message(self, custom_message: Any):
        """
        This method can be used to send a custom message from one
        scene (usually the currently active one) to another scene.
        """

        pass


class GMSceneManager:
    """
    This class manages all the scenes that are used in the engine.
    """

    def __init__(self):
        logger.debug("Create a new GMSceneManager.")

        self.scenes: dict[str, GMScene] = {}
        self.current_scene: GMScene = GMScene("empty")
        self.scene_stack: list[GMScene] = []

    def update(self) -> None:
        """
        Updates the current scene. This method is called by the engine once per frame.
        """

        self.current_scene.update()

    def draw(self) -> None:
        """
        Draws the current scene. This method is called by the engine once per frame.

        :param context: The current game context.
        """

        self.current_scene.draw()

    def add_scene(self, scene: GMScene) -> None:
        """
        Adds a new scene to the scene manager. If a scene with the
        same name already exists it will be replaced.

        :param scene: The new scene to be added.
        """

        assert isinstance(scene, GMScene), "GMSceneManager.add_scene(), new scene must be a subclass of GMScene"

        logger.debug(f"Add a new scene with name: {scene.name}.")

        self.scenes[scene.name] = scene

    def delete_scene(self, name: str) -> None:
        """
        Deletes the scene with the given name.

        :param name: The name of the scene to be deleted.
        :raise KeyError: if the scene with the given name is not found.
        """

        logger.debug(f"Delete scene with name: {name}.")

        del self.scenes[name]

    def change_to_scene(self, name: str) -> None:
        """
        Change the current scene to the given one.

        :param name: The name of the scene to be deleted.
        :raise KeyError: if the scene with the given name is not found.
        """

        logger.debug(f"Change to scene with name: {name}.")

        self.current_scene.leave()
        self.current_scene = self.scenes[name]
        self.current_scene.enter()

    def push_and_change(self, name: str) -> None:
        """
        Push the current scene onto the scene stack and change to the given scene.

        :param name: The name of the scene to be deleted.
        :raise KeyError: if the scene with the given name is not found.
        """

        logger.debug("Push current scene and change to scene with name: {name}.")

        self.scene_stack.append(self.current_scene)
        self.current_scene.on_stack = True
        self.change_to_scene(name)

    def pop_and_change(self) -> None:
        """
        Pops a scene from the scene stack and change to it.

        :raise ValueError: if the stack is empty.
        """

        logger.debug("Pop scene from stack and change to it.")

        self.current_scene.leave()
        self.current_scene = self.scene_stack.pop()
        self.current_scene.on_stack = False
        self.current_scene.enter()

    def start_scene(self, name: str) -> None:
        """
        Sets the first scene to start the game with.

        :param name: The name of the scene to be deleted.
        :raise KeyError: if the scene with the given name is not found.
        """

        self.current_scene = self.scenes[name]
        self.current_scene.enter()

    def update_stack_top(self):
        """
        Update the scene that is on top of the stack.
        """

        self.scene_stack[-1].update()

    def draw_stack_top(self):
        """
        Draw the scene that is on top of the stack.
        """

        self.scene_stack[-1].draw()

    def update_scene(self, name: str):
        """
        Update the specific scene given by the name.

        :param name: The name of the scene to be updated.
        """

        self.scenes[name].update()

    def draw_scene(self, name: str):
        """
        Draw the specific scene given by the name.

        :param name: The name of the scene to be drawn.
        """

        self.scenes[name].draw()

    def send_message(self, name: str, custom_message: Any):
        """
        Sends a custom message to the scene given by the name.

        :param name: The name of the scene to send the custom message to.
        :param custom_message: The custom message that is sent to the scene.
        """

        self.scenes[name].send_message(custom_message)



