# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest
from typing import Any, override

from green_moon_2d.gm_scene import GMScene, GMSceneManager
from green_moon_2d.gm_context import GMContext
from green_moon_2d.gm_messages import GMSceneMessage


class TestScene(GMScene):
    def __init__(self, name):
        super().__init__(name)

        self.custom_property["update_called"] = 0
        self.custom_property["draw_called"] = 0
        self.custom_property["enter_called"] = 0
        self.custom_property["leave_called"] = 0
        self.custom_property["send_message_called"] = 0
        self.custom_property["send_message_value"] = None

    @override
    def update(self, context: GMContext):
        self.custom_property["update_called"] += 1

    @override
    def draw(self, context: GMContext):
        self.custom_property["draw_called"] += 1

    @override
    def enter(self):
        self.custom_property["enter_called"] += 1

    @override
    def leave(self):
        self.custom_property["leave_called"] += 1

    @override
    def send_message(self, custom_message: Any):
        self.custom_property["send_message_called"] += 1
        self.custom_property["send_message_value"] = custom_message


class TestSceneManager(unittest.TestCase):
    def assertSceneProperty(self, name: str, property: str, value):
        self.assertEqual(self.sm.scenes[name].custom_property[property], value)

    def assertSceneCurrent(self, name: str):
        self.assertEqual(self.sm.current_scene.name, name)

    def assertSceneEnter(self, name: str, val: int):
        self.assertEqual(self.sm.scenes[name].custom_property["enter_called"], val)

    def assertSceneLeave(self, name: str, val: int):
        self.assertEqual(self.sm.scenes[name].custom_property["leave_called"], val)

    def assertSceneUpdate(self, name: str, val: int):
        self.assertEqual(self.sm.scenes[name].custom_property["update_called"], val)

    def assertSceneDraw(self, name: str, val: int):
        self.assertEqual(self.sm.scenes[name].custom_property["draw_called"], val)

    def assertSceneStack(self, index: int, name: str):
        self.assertEqual(self.sm.scene_stack[index].name, name)

    def assertSceneOnStack(self, name: str, val: bool):
        self.assertEqual(self.sm.scenes[name].on_stack, val)

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
        self.assertSceneOnStack("test1", True)

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
        self.assertSceneOnStack("test1", False)

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
        self.assertEqual(self.sm.current_scene.name, "empty")

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

    def test_update_stack_top(self):
        """
        Test calling draw on the current scene.
        """
        self.sm.add_scene(TestScene("test1"))
        self.sm.add_scene(TestScene("test2"))
        self.sm.start_scene("test1")

        self.assertSceneUpdate("test1", 0)
        self.assertSceneUpdate("test2", 0)

        context = GMContext()
        self.sm.push_and_change("test2")
        self.sm.update_stack_top(context)

        self.assertSceneUpdate("test1", 1)
        self.assertSceneUpdate("test2", 0)

    def test_draw_stack_top(self):
        raise NotImplementedError
        # TODO: implement test case.

    def test_update_scene_name(self):
        raise NotImplementedError
        # TODO: implement test case.

    def test_draw_scene_name(self):
        raise NotImplementedError
        # TODO: implement test case.

    def test_send_custom_message(self):
        raise NotImplementedError
        # TODO: implement test case.

    def test_scene_messages1(self):
        """
        Test scene messages: add
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("add", scene=TestScene("Sally")))
        self.assertEqual(len(self.sm.scenes), 0)
        self.sm.update(context)
        self.assertEqual(len(context.scene_messages), 0)
        self.assertEqual(len(self.sm.scenes), 1)
        self.assertIn("Sally", self.sm.scenes)

    def test_scene_messages2(self):
        """
        Test scene messages: delete
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("add", scene=TestScene("Sally")))
        self.assertEqual(len(self.sm.scenes), 0)
        self.sm.update(context)
        self.assertEqual(len(context.scene_messages), 0)
        self.assertEqual(len(self.sm.scenes), 1)
        self.assertIn("Sally", self.sm.scenes)

        context.scene_messages.append(GMSceneMessage("delete", scene_name="Sally"))
        self.sm.update(context)
        self.assertEqual(len(self.sm.scenes), 0)

    def test_scene_messages3(self):
        """
        Test scene messages: change
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("add", scene=TestScene("Sally")))
        context.scene_messages.append(GMSceneMessage("add", scene=TestScene("Bobby")))
        self.sm.update(context)
        self.assertEqual(len(self.sm.scenes), 2)
        self.assertIn("Sally", self.sm.scenes)
        self.assertIn("Bobby", self.sm.scenes)

        self.sm.start_scene("Sally")
        self.assertSceneCurrent("Sally")
        self.assertSceneEnter("Sally", 1)

        context.scene_messages.append(GMSceneMessage("change", scene_name="Bobby"))
        self.sm.update(context)
        self.assertSceneCurrent("Bobby")
        self.assertSceneEnter("Sally", 1)
        self.assertSceneLeave("Sally", 1)
        self.assertSceneEnter("Bobby", 1)

    def test_scene_messages4(self):
        """
        Test scene messages: push
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("add", scene=TestScene("Sally")))
        context.scene_messages.append(GMSceneMessage("add", scene=TestScene("Bobby")))
        context.scene_messages.append(GMSceneMessage("add", scene=TestScene("Zed")))
        self.sm.update(context)
        self.sm.start_scene("Sally")

        context.scene_messages.append(GMSceneMessage("push", scene_name="Zed"))
        self.sm.update(context)
        self.assertSceneEnter("Sally", 1)
        self.assertSceneLeave("Sally", 1)
        self.assertSceneEnter("Zed", 1)
        self.assertSceneLeave("Zed", 0)
        self.assertSceneCurrent("Zed")
        self.assertEqual(len(self.sm.scene_stack), 1)
        self.assertSceneStack(0, "Sally")

    def test_scene_messages5(self):
        """
        Test scene messages: pop
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("add", scene=TestScene("Sally")))
        context.scene_messages.append(GMSceneMessage("add", scene=TestScene("Bobby")))
        context.scene_messages.append(GMSceneMessage("add", scene=TestScene("Zed")))
        self.sm.update(context)
        self.sm.start_scene("Sally")

        context.scene_messages.append(GMSceneMessage("push", scene_name="Bobby"))
        self.sm.update(context)
        self.assertSceneCurrent("Bobby")
        self.assertEqual(len(self.sm.scene_stack), 1)
        self.assertSceneStack(0, "Sally")

        context.scene_messages.append(GMSceneMessage("push", scene_name="Zed"))
        self.sm.update(context)
        self.assertSceneCurrent("Zed")
        self.assertEqual(len(self.sm.scene_stack), 2)
        self.assertSceneStack(0, "Sally")
        self.assertSceneStack(1, "Bobby")

        self.assertSceneEnter("Sally", 1)
        self.assertSceneLeave("Sally", 1)
        self.assertSceneEnter("Bobby", 1)
        self.assertSceneLeave("Bobby", 1)
        self.assertSceneEnter("Zed", 1)
        self.assertSceneLeave("Zed", 0)

        context.scene_messages.append(GMSceneMessage("pop"))
        self.sm.update(context)
        self.assertSceneCurrent("Bobby")
        self.assertEqual(len(self.sm.scene_stack), 1)
        self.assertSceneStack(0, "Sally")

        self.assertSceneEnter("Sally", 1)
        self.assertSceneLeave("Sally", 1)
        self.assertSceneEnter("Bobby", 2)
        self.assertSceneLeave("Bobby", 1)
        self.assertSceneEnter("Zed", 1)
        self.assertSceneLeave("Zed", 1)

        context.scene_messages.append(GMSceneMessage("pop"))
        self.sm.update(context)
        self.assertSceneCurrent("Sally")
        self.assertEqual(len(self.sm.scene_stack), 0)

        self.assertSceneEnter("Sally", 2)
        self.assertSceneLeave("Sally", 1)
        self.assertSceneEnter("Bobby", 2)
        self.assertSceneLeave("Bobby", 2)
        self.assertSceneEnter("Zed", 1)
        self.assertSceneLeave("Zed", 1)

    def test_scene_messages6(self):
        """
        Test scene messages: update_stack_top
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("update_stack_top"))

        raise NotImplementedError
        # TODO: implement test case.

    def test_scene_messages7(self):
        """
        Test scene messages: draw_stack_top
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("draw_stack_top"))

        raise NotImplementedError
        # TODO: implement test case.

    def test_scene_messages8(self):
        """
        Test scene messages: update_scene
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("update_scene", scene_name="test2"))

        raise NotImplementedError
        # TODO: implement test case.

    def test_scene_messages9(self):
        """
        Test scene messages: draw_scene
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("draw_scene", scene_name="test2"))

        raise NotImplementedError
        # TODO: implement test case.

    def test_scene_messages10(self):
        """
        Test scene messages: send_message
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("send_message", custom_message="Foo1"))

        raise NotImplementedError
        # TODO: implement test case.

    def test_scene_messages11(self):
        """
        Test scene messages: unknown
        """

        context = GMContext()
        context.scene_messages.append(GMSceneMessage("the_unknown"))

        with self.assertRaises(ValueError):
            self.sm.update(context)


if __name__ == "__main__":
    unittest.main()

