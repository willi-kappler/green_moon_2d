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
        Test adding a scene with the same name (replace).
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
        self.assertEqual(self.sm.current_scene.name, "test2")
        self.assertEqual(self.sm.current_scene.enter_called, 1)
        self.assertEqual(self.sm.current_scene.leave_called, 0)
        self.assertEqual(self.sm.scenes["test1"].enter_called, 0)
        self.assertEqual(self.sm.scenes["test1"].leave_called, 0)
        self.assertEqual(self.sm.scenes["test3"].enter_called, 0)
        self.assertEqual(self.sm.scenes["test3"].leave_called, 0)

        self.sm.change_to_scene("test3")
        self.assertEqual(self.sm.current_scene.name, "test3")
        self.assertEqual(self.sm.current_scene.enter_called, 1)
        self.assertEqual(self.sm.current_scene.leave_called, 0)
        self.assertEqual(self.sm.scenes["test1"].enter_called, 0)
        self.assertEqual(self.sm.scenes["test1"].leave_called, 0)
        self.assertEqual(self.sm.scenes["test2"].enter_called, 1)
        self.assertEqual(self.sm.scenes["test2"].leave_called, 1)

    def test_change_to_scene1(self):
        """
        Test change to scene where name is not found.
        """
        with self.assertRaises(KeyError):
            self.sm.change_to_scene("test1")

    def test_push_and_change1(self):
        """
        Test push and change scene.
        """
        pass

    def test_push_and_change2(self):
        """
        Test push and change scene where name is not found.
        """
        pass

    def test_pop_and_change1(self):
        """
        Test pop and change scene.
        """
        pass

    def test_pop_and_change2(self):
        """
        Test pop and change scene where the scene stack is empty.
        """
        pass

    def test_start_scene(self):
        """
        Test setting the start scene.
        """
        pass

    def test_update(self):
        """
        Test calling update on the current scene.
        """
        pass

    def test_update(self):
        """
        Test calling draw on the current scene.
        """
        pass


if __name__ == '__main__':
    unittest.main()

