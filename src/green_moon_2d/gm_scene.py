# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMSceneManager and the GMScene base class.
"""

from green_moon_2d.gm_context import GMContext

class GMScene:
    """
    This is the base class for a user defined scene.
    You have to provide a scene name and imlement the update() and draw() methods.
    """

    def __init__(self, name: str):
        self.name = name

    def update(self, context: GMContext) -> None:
        """
        This method is called once per frame and is used to update the internal state of the scene.
        You have to implement this method.
        """
        pass

    def draw(self, context: GMContext) -> None:
        """
        This method is called once per frame and is used to draw the whole scene.
        You have to implement this method.
        """
        pass

    def enter(self) -> None:
        """
        This method is called every time this scene becomes the current scene.
        You don't have to implement this mwthod.
        """
        pass

    def leave(self) -> None:
        """
        This method is called every time this scene is no longer the current scene.
        You don't have to implement this mwthod.
        """
        pass


class GMSceneManager:
    """
    This class manages all the scenes that are used in the engine.
    """

    def __init__(self):
        self.scenes = {}
        self.current_scene = None
        self.scene_stack = []

    def update(self, context: GMContext) -> None:
        """
        Updates the current scene. This method is called by the engine once per frame.
        """
        self.current_scene.update(context)

    def draw(self, context: GMContext) -> None:
        """
        Draws the current scene. This method is called by the engine once per frame.
        """
        self.current_scene.draw(context)

    def add_scene(self, scene: GMScene) -> None:
        """
        Adds a new scene to the scene manager. If a scene with the same name already exists it will be replaced.
        """
        assert isinstance(scene, GMScene), "GMSceneManager.add_scene(), new scene must be a subclass of GMScene"

        self.scenes[scene.name] = scene

    def delete_scene(self, name: str) -> None:
        """
        Deletes the scene with the given name. If the scene is not found raise an exception.
        """

        del self.scenes[name]

    def change_to_scene(self, name: str) -> None:
        """
        Change the current scene to the given one. If the scene is not found raise an exception.
        """

        self.current_scene.leave()
        self.current_scene = self.scenes[name]
        self.current_scene.enter()

    def push_and_change(self, name: str) -> None:
        """
        Push the current scene onto the scene stack and change to the given scene.
        If the scene is not found raise an exception.
        """
        self.scene_stack.add(self.current_scene)
        self.change_to_scene(name)

    def pop_and_change(self) -> None:
        """
        Pops a scene from the scene stack and change to it. If the stack is empty raise an exception.
        """
        self.current_scene.leave()
        self.current_scene = self.scene_stack.pop()
        self.current_scene.enter()



