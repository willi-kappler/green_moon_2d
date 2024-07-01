# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest

from green_moon_2d.gm_scene import GMScene, GMSceneManager
from green_moon_2d.gm_context import GMContext

class TestScene(GMScene):
    def __init__(self, name):
        super().__init__(name)

        self.update_called = 0
        self.draw_called = 0
        self.enter_called = 0
        self.leave_called = 0

    def update(self, context: GMContext):
        self.update_called += 1

    def draw(self, context: GMContext):
        self.draw_called += 1

    def enter(self):
        self.enter_called += 1

    def leave(self):
        self.leave_called += 1

class TestSceneManager(unittest.TestCase):
    def assertSceneProperty(self, name: str, property: str, value):
        self.assertEqual(self.sm.scenes[name].custom_property[property], value)

    def assertSceneCurrent(self, name: str):
        self.assertEqual(self.sm.current_scene.name, name)

    def assertSceneEnter(self, name: str, val: int):
        self.assertEqual(self.sm.scenes[name].enter_called, val)

    def assertSceneLeave(self, name: str, val: int):
        self.assertEqual(self.sm.scenes[name].leave_called, val)

    def assertSceneUpdate(self, name: str, val: int):
        self.assertEqual(self.sm.scenes[name].update_called, val)

    def assertSceneDraw(self, name: str, val: int):
        self.assertEqual(self.sm.scenes[name].draw_called, val)

    def assertSceneStack(self, index: int, name: str):
        self.assertEqual(self.sm.scene_stack[index].name, name)

    def setUp(self):
        self.sm = GMSceneManager()

    def test_add_scene1(self):
        """
        Test adding one scene.
        """
        self.assertEqual(len(self.sm.scenes), 0)
        self.sm.add_scene(TestScene("test1"))
        self.assertEqual(len(self.sm.scenes), 1)
        self.assertIn("test1", self.sm.scenes)

    def test_add_scene2(self):
        """
        Test adding two scenes.
        """
        self.assertEqual(len(self.sm.scenes), 0)
        self.sm.add_scene(TestScene("test1"))
        self.assertEqual(len(self.sm.scenes), 1)
        self.assertIn("test1", self.sm.scenes)

        self.sm.add_scene(TestScene("test2"))
        self.assertEqual(len(self.sm.scenes), 2)
        self.assertIn("test1", self.sm.scenes)
        self.assertIn("test2", self.sm.scenes)

    def test_add_scene3(self):
        """
        Test adding a scene with the same name (replace).
        """
        new_scene = TestScene("test1")
        new_scene.custom_property["replaced"] = "No"
        self.sm.add_scene(new_scene)
        self.assertSceneProperty("test1", "replaced", "No")

        new_scene = TestScene("test1")
        new_scene.custom_property["replaced"] = "Yes"
        self.sm.add_scene(new_scene)
        self.assertSceneProperty("test1", "replaced", "Yes")

    def test_delete_scene1(self):
        """
        Test delete scene.
        """
        self.sm.add_scene(TestScene("test1"))
        self.sm.delete_scene("test1")
        self.assertEqual(len(self.sm.scenes), 0)

    def test_delete_scene2(self):
        """
        Test delete scene where name is not found.
        """
        self.sm.add_scene(TestScene("test1"))

        with self.assertRaises(KeyError):
            self.sm.delete_scene("test2")

        self.assertEqual(len(self.sm.scenes), 1)
        self.assertIn("test1", self.sm.scenes)

    def test_change_to_scene1(self):
        """
        Test change to scene.
        """
        self.sm.add_scene(TestScene("test1"))
        self.sm.add_scene(TestScene("test2"))
        self.sm.add_scene(TestScene("test3"))

        self.sm.start_scene("test2")
        self.assertSceneCurrent("test2")
        self.assertSceneEnter("test1", 0)
        self.assertSceneLeave("test1", 0)
        self.assertSceneEnter("test2", 1)
        self.assertSceneLeave("test2", 0)
        self.assertSceneEnter("test3", 0)
        self.assertSceneLeave("test3", 0)

        self.sm.change_to_scene("test3")
        self.assertSceneCurrent("test3")
        self.assertSceneEnter("test1", 0)
        self.assertSceneLeave("test1", 0)
        self.assertSceneEnter("test2", 1)
        self.assertSceneLeave("test2", 1)
        self.assertSceneEnter("test3", 1)
        self.assertSceneLeave("test3", 0)

    def test_change_to_scene2(self):
        """
        Test change to scene where name is not found.
        """
        self.sm.add_scene(TestScene("test1"))
        self.sm.start_scene("test1")

        with self.assertRaises(KeyError):
            self.sm.change_to_scene("test2")

    def test_push_and_change1(self):
        """
        Test push and change scene.
        """
        self.sm.add_scene(TestScene("test1"))
        self.sm.add_scene(TestScene("test2"))
        self.sm.add_scene(TestScene("test3"))

        self.assertEqual(len(self.sm.scene_stack), 0)

        self.sm.start_scene("test1")
        self.sm.push_and_change("test2")
        self.assertSceneEnter("test1", 1)
        self.assertSceneLeave("test1", 1)
        self.assertSceneEnter("test2", 1)
        self.assertSceneLeave("test2", 0)
        self.assertSceneCurrent("test2")
        self.assertEqual(len(self.sm.scene_stack), 1)
        self.assertSceneStack(0, "test1")

    def test_push_and_change2(self):
        """
        Test push and change scene where name is not found.
        """
        self.sm.add_scene(TestScene("test1"))
        self.sm.add_scene(TestScene("test2"))
        self.sm.add_scene(TestScene("test3"))
        self.sm.start_scene("test1")

        with self.assertRaises(KeyError):
            self.sm.push_and_change("test5")

    def test_pop_and_change1(self):
        """
        Test pop and change scene.
        """
        self.sm.add_scene(TestScene("test1"))
        self.sm.add_scene(TestScene("test2"))
        self.sm.add_scene(TestScene("test3"))
        self.sm.start_scene("test1")
        self.sm.push_and_change("test2")

        self.sm.pop_and_change()
        self.assertSceneEnter("test1", 2)
        self.assertSceneLeave("test1", 1)
        self.assertSceneEnter("test2", 1)
        self.assertSceneLeave("test2", 1)
        self.assertSceneCurrent("test1")

    def test_pop_and_change2(self):
        """
        Test pop and change scene where the scene stack is empty.
        """
        self.sm.add_scene(TestScene("test1"))
        self.sm.start_scene("test1")

        with self.assertRaises(IndexError):
            self.sm.pop_and_change()

    def test_start_scene(self):
        """
        Test setting the start scene.
        """
        self.sm.add_scene(TestScene("test1"))
        self.assertEqual(self.sm.current_scene, None)

        self.sm.start_scene("test1")
        self.assertSceneCurrent("test1")

    def test_update(self):
        """
        Test calling update on the current scene.
        """
        self.sm.add_scene(TestScene("test1"))
        self.sm.add_scene(TestScene("test2"))
        self.sm.start_scene("test1")

        self.assertSceneUpdate("test1", 0)
        self.assertSceneUpdate("test2", 0)

        context = GMContext()
        self.sm.update(context)
        self.assertSceneUpdate("test1", 1)
        self.assertSceneUpdate("test2", 0)
        self.sm.change_to_scene("test2")
        self.sm.update(context)
        self.assertSceneUpdate("test1", 1)
        self.assertSceneUpdate("test2", 1)

    def test_draw(self):
        """
        Test calling draw on the current scene.
        """
        self.sm.add_scene(TestScene("test1"))
        self.sm.add_scene(TestScene("test2"))
        self.sm.start_scene("test1")

        self.assertSceneDraw("test1", 0)
        self.assertSceneDraw("test2", 0)

        context = GMContext()
        self.sm.draw(context)
        self.assertSceneDraw("test1", 1)
        self.assertSceneDraw("test2", 0)
        self.sm.change_to_scene("test2")
        self.sm.draw(context)
        self.assertSceneDraw("test1", 1)
        self.assertSceneDraw("test2", 1)

if __name__ == '__main__':
    unittest.main()

