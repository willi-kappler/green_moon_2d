# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMSceneManager and the GMScene base class.
"""

from typing import Any

from green_moon_2d.gm_context import GMContext
from green_moon_2d.gm_interfaces import GMSceneInterface


class GMScene(GMSceneInterface):
    """
    This is the base class for a user defined scene.
    You have to provide a scene name and imlement the update() and draw() methods accordingly.
    """

    def __init__(self, name: str):
        """
        :param name: The name of the new scene. It must be unique.
        """

        self.name: str = name
        self.on_stack: bool = False
        self.custom_property: dict[str, Any] = {}

    def update(self, context: GMContext) -> None:
        """
        This method is called once per frame and is used to update the
        internal state of the scene. You have to implement this method.

        :param context: The current game context.
        """

        pass

    def draw(self, context: GMContext) -> None:
        """
        This method is called once per frame and is used to draw the
        whole scene. You have to implement this method.

        :param context: The current game context.
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
        self.scenes: dict[str, GMScene] = {}
        self.current_scene: GMScene = GMScene("empty")
        self.scene_stack: list[GMScene] = []

    def update(self, context: GMContext) -> None:
        """
        Updates the current scene. This method is called by the engine once per frame.
        The scene manager also processes all the scene messages here.

        :param context: The current game context.
        """

        self.current_scene.update(context)

        for msg in context.scene_messages:
            match msg.kind:
                case "add":
                    if isinstance(msg.scene, GMScene):
                        self.add_scene(msg.scene)
                    else:
                        raise ValueError(f"Scene message add scene must be of type GMScene: {msg.scene}")
                case "delete":
                    self.delete_scene(msg.scene_name)
                case "change":
                    self.change_to_scene(msg.scene_name)
                case "push":
                    self.push_and_change(msg.scene_name)
                case "pop":
                    self.pop_and_change()
                case "update_stack_top":
                    self.update_stack_top(context)
                case "draw_stack_top":
                    self.draw_stack_top(context)
                case "update_scene":
                    self.update_scene(msg.scene_name, context)
                case "draw_scene":
                    self.draw_scene(msg.scene_name, context)
                case "send_message":
                    self.send_message(msg.scene_name, msg.custom_message)
                case _:
                    raise ValueError(f"Unknown scene message kind: {msg.kind}.")

        context.scene_messages.clear()

    def draw(self, context: GMContext) -> None:
        """
        Draws the current scene. This method is called by the engine once per frame.

        :param context: The current game context.
        """

        self.current_scene.draw(context)

    def add_scene(self, scene: GMScene) -> None:
        """
        Adds a new scene to the scene manager. If a scene with the
        same name already exists it will be replaced.

        :param scene: The new scene to be added.
        """

        assert isinstance(scene, GMScene), "GMSceneManager.add_scene(), new scene must be a subclass of GMScene"

        self.scenes[scene.name] = scene

    def delete_scene(self, name: str) -> None:
        """
        Deletes the scene with the given name.

        :param name: The name of the scene to be deleted.
        :raise KeyError: if the scene with the given name is not found.
        """

        del self.scenes[name]

    def change_to_scene(self, name: str) -> None:
        """
        Change the current scene to the given one.

        :param name: The name of the scene to be deleted.
        :raise KeyError: if the scene with the given name is not found.
        """

        self.current_scene.leave()
        self.current_scene = self.scenes[name]
        self.current_scene.enter()

    def push_and_change(self, name: str) -> None:
        """
        Push the current scene onto the scene stack and change to the given scene.

        :param name: The name of the scene to be deleted.
        :raise KeyError: if the scene with the given name is not found.
        """

        self.scene_stack.append(self.current_scene)
        self.current_scene.on_stack = True
        self.change_to_scene(name)

    def pop_and_change(self) -> None:
        """
        Pops a scene from the scene stack and change to it.

        :raise ValueError: if the stack is empty.
        """

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

    def update_stack_top(self, context: GMContext):
        """
        Update the scene that is on top of the stack.

        :param context: The current game context.
        """

        self.scene_stack[-1].update(context)

    def draw_stack_top(self, context: GMContext):
        """
        Draw the scene that is on top of the stack.

        :param context: The current game context.
        """

        self.scene_stack[-1].draw(context)

    def update_scene(self, name: str, context: GMContext):
        """
        Update the specific scene given by the name.

        :param name: The name of the scene to be updated.
        :param context: The current game context.
        """

        self.scenes[name].update(context)

    def draw_scene(self, name: str, context: GMContext):
        """
        Draw the specific scene given by the name.

        :param name: The name of the scene to be drawn.
        :param context: The current game context.
        """

        self.scenes[name].draw(context)

    def send_message(self, name: str, custom_message: Any):
        """
        Sends a custom message to the scene given by the name.

        :param name: The name of the scene to send the custom message to.
        :param custom_message: The custom message that is sent to the scene.
        """

        self.scenes[name].send_message(custom_message)



