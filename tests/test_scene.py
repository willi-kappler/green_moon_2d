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
        Test adding a scene with the same name.
        """
        new_scene = TestScene("test1")
        new_scene.custom_property["replaced"] = "No"
        self.sm.add_scene(new_scene)
        self.assertEqual(self.sm.scenes["test1"].custom_property["replaced"], "No")

        new_scene = TestScene("test1")
        new_scene.custom_property["replaced"] = "Yes"
        self.sm.add_scene(new_scene)
        self.assertEqual(self.sm.scenes["test1"].custom_property["replaced"], "Yes")

    def test_delete_scene1(self):
        self.sm.add_scene(TestScene("test1"))
        self.sm.delete_scene("test1")
        self.assertEqual(len(self.sm.scenes), 0)

    def test_delete_scene2(self):
        self.sm.add_scene(TestScene("test1"))

        with self.assertRaises(KeyError):
            self.sm.delete_scene("test2")

        self.assertEqual(len(self.sm.scenes), 1)
        self.assertIn("test1", self.sm.scenes)


if __name__ == '__main__':
    unittest.main()

