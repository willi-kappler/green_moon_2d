# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler
#
# See: https://github.com/willi-kappler/green_moon_2d
#
# This module defines the GMSceneManager and the GMScene base class.


class GMScene:
    def __init__(self, name):
        self.name = name

    def update(self, context):
        pass

    def draw(self, context):
        pass

class GMSceneManager:
    def __init__(self):
        self.scenes = []
        self.current_scene = None
        self.scene_stack = []

    def update(self, context):
        self.current_scene.update(context)

    def draw(self, context):
        self.current_scene.draw(context)

    def add_scene(self, scene):
        assert isinstance(scene, GMScene), f"GMSceneManager.add_scene(), new scene must be a subclass of GMScene"

        for s in self.scenes:
            if scene.name = s.name:
                raise ValueError(f"GMSceneManager.add_scene(), a scene with that name already exists: {scene.name}")

        self.scenes.add(scene)

    def delete_scene(self, name):
        for s in self.scenes:
            pass
            # TODO: Implement

    def change_to_scene(self, name):
        for s in self.scenes:
            if s.name == name:
                self.current_scene = s
                return

        raise ValueError(f"GMSceneManager.change_to_scene(), a scene with that name was not found: {name}")

    def push_and_change(self, name):
        prev_scene = self.current_scene
        self.change_to_scene(name)
        self.scene_stack.add(prev_scene)

    def pop_and_change(self):
        self.current_scene = self.scene_stack.pop()





