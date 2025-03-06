# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest

from typing import Any, override

from green_moon_2d.gm_message import GMMessage
from green_moon_2d.gm_object import GMObject, GMObjectManager
# from green_moon_2d.gm_math import GMVec2D


class TestObject(GMObject):
    def __init__(self, name):
        super().__init__(name)

        self.properties["update_called"] = 0
        self.properties["draw_called"] = 0

    @override
    def update(self, dt: float, om: GMObjectManager):
        self.properties["update_called"] += 1

    @override
    def draw(self):
        self.properties["draw_called"] += 1


class TestObjectManager(unittest.TestCase):
    def setUp(self):
        self.om = GMObjectManager()

    def assertNumObjects(self, n: int) -> None:
        self.assertEqual(len(self.om.objects), n)

    def assertObjectIndex(self, name: str, n: int) -> None:
        self.assertEqual(self.om.get_index(name), n)

    def assertObjectGroups(self, name: str, groups: list[str]) -> None:
        self.assertEqual(self.om[name].groups, set(groups))

    def assertObjectProperty(self, name: str, property: str, val: Any):
        self.assertEqual(self.om[name].properties[property], val)

    def test_add_object1(self):
        """
        Test adding a new object.
        """

        # self.assertEqual(len(self.om.objects), 0)
        self.assertNumObjects(0)
        self.om.add(TestObject("test1"))
        # self.assertEqual(self.om.get_index("test1"), 0)
        self.assertObjectIndex("test1", 0)
        # self.assertEqual(len(self.om.objects), 1)
        self.assertNumObjects(1)

    def test_add_object2(self):
        """
        Test adding a new object with a used name.
        """

        self.om.add(TestObject("test1"))
        with self.assertRaises(KeyError):
            self.om.add(TestObject("test1"))

    def test_delete_object1(self):
        """
        Test deleting an object.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        self.assertObjectIndex("test1", 0)
        self.assertObjectIndex("test2", 1)
        self.assertNumObjects(2)

        self.om.delete("test1")
        self.assertObjectIndex("test2", 0)
        self.assertNumObjects(1)

    def test_delete_object2(self):
        """
        Test deleting an object that doesn't exist.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        with self.assertRaises(KeyError):
            self.om.delete("test3")

    def test_sort_update(self):
        """
        Test sorting using the update order.
        """

        o1 = TestObject("test1")
        o1.update_order = 2
        o2 = TestObject("test2")
        o2.update_order = 1
        o3 = TestObject("test3")
        o3.update_order = 3

        self.om.add([o1, o2, o3])
        self.assertObjectIndex("test1", 0)
        self.assertObjectIndex("test2", 1)
        self.assertObjectIndex("test3", 2)

        self.om.sort_update()
        self.assertObjectIndex("test2", 0)
        self.assertObjectIndex("test1", 1)
        self.assertObjectIndex("test3", 2)

    def test_sort_draw(self):
        """
        Test sorting useing the draw order.
        """

        o1 = TestObject("test1")
        o1.draw_order = 2
        o2 = TestObject("test2")
        o2.draw_order = 1
        o3 = TestObject("test3")
        o3.draw_order = 3

        self.om.add([o1, o2, o3])
        self.assertObjectIndex("test1", 0)
        self.assertObjectIndex("test2", 1)
        self.assertObjectIndex("test3", 2)

        self.om.sort_draw()
        self.assertObjectIndex("test2", 0)
        self.assertObjectIndex("test1", 1)
        self.assertObjectIndex("test3", 2)

    def test_get_index(self):
        """
        Test get the index of an object.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        self.assertEqual(self.om.get_index("test1"), 0)
        self.assertEqual(self.om.get_index("test2"), 1)
        self.assertEqual(self.om.get_index("test3"), 2)
        self.assertEqual(self.om.get_index("test4"), None)

    def test_get1(self):
        """
        Test get object by name.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        obj = self.om["test1"]
        self.assertEqual(obj.name, "test1")
        obj = self.om["test2"]
        self.assertEqual(obj.name, "test2")
        obj = self.om["test3"]
        self.assertEqual(obj.name, "test3")

    def test_get2(self):
        """
        Test get object ba name with an unknown name.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        with self.assertRaises(KeyError):
            self.om["test4"]

    def test_update(self):
        """
        Test calling update on all objects.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        self.om.update(0.0)

        self.assertObjectProperty("test1", "update_called", 1)
        self.assertObjectProperty("test2", "update_called", 1)

        self.om.add(TestObject("test3"))
        self.om.update(0.0)

        self.assertObjectProperty("test1", "update_called", 2)
        self.assertObjectProperty("test2", "update_called", 2)
        self.assertObjectProperty("test3", "update_called", 1)

    def test_draw(self):
        """
        Test calling draw on all objects.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        self.om.draw()

        self.assertObjectProperty("test1", "draw_called", 1)
        self.assertObjectProperty("test2", "draw_called", 1)

        self.om.add(TestObject("test3"))
        self.om.draw()

        self.assertObjectProperty("test1", "draw_called", 2)
        self.assertObjectProperty("test2", "draw_called", 2)
        self.assertObjectProperty("test3", "draw_called", 1)

    def test_send_message1(self):
        """
        Test sending a message to an object.
        """

        self.om.add(TestObject("test1"))
        self.assertEqual(len(self.om["test1"].properties), 2)

        msg = GMMessage.single("test1", "set_property", ("message", "alpha_message"))
        self.om.send_message(msg)
        self.assertObjectProperty("test1", "message", "alpha_message")

    def test_send_message2(self):
        """
        Test sending a message to an unknown object.
        """

        self.om.add(TestObject("test1"))

        with self.assertRaises(KeyError):
            msg = GMMessage.single("test2", "foo", None)
            self.om.send_message(msg)

    def send_message_group1(self):
        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))
        self.om.add(TestObject("test4"))

        self.om.add_group("test1", "foo")
        self.om.add_group("test2", "foo")

        msg = GMMessage.group("foo", "beta_message", None)
        self.om.send_message(msg)

        self.assertObjectProperty("test1", "message", "beta_message")
        self.assertObjectProperty("test2", "message", "beta_message")

        self.assertFalse(self.om.has_property("test3", "message"))
        self.assertFalse(self.om.has_property("test4", "message"))

    def test_add_group1(self):
        """
        Test adding groups to objects.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        self.assertObjectGroups("test1", [])
        self.assertObjectGroups("test2", [])

        self.om.add_group("test1", "foo")

        self.assertObjectGroups("test1", ["foo"])
        self.assertObjectGroups("test2", [])

        self.om.add_groups("test1", ["bar", "baz"])
        self.om.add_group("test2", "green")

        self.assertObjectGroups("test1", ["foo", "bar", "baz"])
        self.assertObjectGroups("test2", ["green"])

    def test_add_group2(self):
        """
        Test adding a group to an unknown object.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        with self.assertRaises(KeyError):
            self.om.add_group("test3", "bar")

    def test_remove_group1(self):
        """
        Test removing a group from an object.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add_groups("test1", ["foo", "bar"])
        self.om.add_groups("test2", ["green", "blue", "yellow"])

        self.assertObjectGroups("test1", ["foo", "bar"])
        self.assertObjectGroups("test2", ["green", "blue", "yellow"])

        self.om.remove_group("test1", "foo")

        self.assertObjectGroups("test1", ["bar"])
        self.assertObjectGroups("test2", ["green", "blue", "yellow"])

        self.om.remove_group("test2", "yellow")

        self.assertObjectGroups("test1", ["bar"])
        self.assertObjectGroups("test2", ["green", "blue"])

        self.om.remove_group("test2", "yellow")

        self.assertObjectGroups("test1", ["bar"])
        self.assertObjectGroups("test2", ["green", "blue"])

    def test_remove_group2(self):
        """
        Test removing a group from an unknown object.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        with self.assertRaises(KeyError):
            self.om.remove_group("test3", "foo")

    def test_remove_group_from_all(self):
        """
        Test removing a group from all objects
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        self.om.add_groups("test1", ["foo", "bar", "top"])
        self.om.add_groups("test2", ["green", "blue", "yellow", "top"])
        self.om.add_group("test3", "poison")

        self.assertObjectGroups("test1", ["foo", "bar", "top"])
        self.assertObjectGroups("test2", ["green", "blue", "yellow", "top"])
        self.assertObjectGroups("test3", ["poison"])

        self.om.remove_group_from_all("top")

        self.assertObjectGroups("test1", ["foo", "bar"])
        self.assertObjectGroups("test2", ["green", "blue", "yellow"])
        self.assertObjectGroups("test3", ["poison"])

    def test_clear_groups1(self):
        """
        Test removing all groups from an object.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        self.om.add_groups("test1", ["foo", "bar", "top"])
        self.om.add_groups("test2", ["green", "blue", "yellow", "top"])

        self.assertObjectGroups("test1", ["foo", "bar", "top"])
        self.assertObjectGroups("test2", ["green", "blue", "yellow", "top"])

        self.om.clear_groups("test1")

        self.assertObjectGroups("test1", [])
        self.assertObjectGroups("test2", ["green", "blue", "yellow", "top"])

    def test_clear_groups2(self):
        """
        Test removing all groups from an unknown object.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        with self.assertRaises(KeyError):
            self.om.clear_groups("test3")

    def test_iter(self):
        """
        Test iterating over all objects.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        self.om["test1"].properties["iter_ok"] = "No"
        self.om["test2"].properties["iter_ok"] = "No"
        self.om["test3"].properties["iter_ok"] = "No"

        for ob in self.om:
            ob.properties["iter_ok"] = "Yes"

        self.assertObjectProperty("test1", "iter_ok", "Yes")
        self.assertObjectProperty("test2", "iter_ok", "Yes")
        self.assertObjectProperty("test3", "iter_ok", "Yes")

    def test_iter_group1(self):
        """
        Test iterating over all objects that belong to a group.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        self.om.add_groups("test1", ["foo", "bar", "top"])
        self.om.add_groups("test2", ["green", "blue", "yellow"])
        self.om.add_groups("test3", ["bleeding", "top"])

        self.om["test1"].properties["Called"] = "No"
        self.om["test2"].properties["Called"] = "No"
        self.om["test3"].properties["Called"] = "No"

        for ob in self.om.iter_group("top"):
            ob.properties["Called"] = "Yes"

        self.assertObjectProperty("test1", "Called", "Yes")
        self.assertObjectProperty("test2", "Called", "No")
        self.assertObjectProperty("test3", "Called", "Yes")

    def test_iter_group2(self):
        """
        Test iteration over all objects that belong to an unknown group.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        self.om.add_groups("test1", ["foo", "bar", "top"])
        self.om.add_groups("test2", ["blue", "top"])

        self.om["test1"].properties["Called"] = "No"
        self.om["test2"].properties["Called"] = "No"

        for ob in self.om.iter_group("yellow"):
            ob.properties["Called"] = "Yes"

        self.assertObjectProperty("test1", "Called", "No")
        self.assertObjectProperty("test2", "Called", "No")

    def test_apply_group1(self):
        """
        Test applying a function to all objects of a group.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        self.om.add_groups("test1", ["foo", "bar", "top"])
        self.om.add_groups("test2", ["green", "blue", "yellow"])
        self.om.add_groups("test3", ["bleeding", "top"])

        self.om["test1"].properties["Called"] = "No"
        self.om["test2"].properties["Called"] = "No"
        self.om["test3"].properties["Called"] = "No"

        self.om.apply_group("top", lambda ob: ob.set_property("Called", "Yes"))

        self.assertObjectProperty("test1", "Called", "Yes")
        self.assertObjectProperty("test2", "Called", "No")
        self.assertObjectProperty("test3", "Called", "Yes")

    def test_apply_group2(self):
        """
        Test applying a function to all object of an unknown group.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        self.om.add_groups("test1", ["foo", "bar", "top"])
        self.om.add_groups("test2", ["green", "blue", "yellow"])
        self.om.add_groups("test3", ["bleeding", "top"])

        self.om["test1"].properties["Called"] = "No"
        self.om["test2"].properties["Called"] = "No"
        self.om["test3"].properties["Called"] = "No"

        self.om.apply_group(
            "flop", lambda ob: ob.set_property("Called", "Yes")
        )

        self.assertObjectProperty("test1", "Called", "No")
        self.assertObjectProperty("test2", "Called", "No")
        self.assertObjectProperty("test3", "Called", "No")

    def test_collect_group1(self):
        """
        Test collecting data from all objects of a group.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        self.om.add_groups("test1", ["foo", "bar", "top"])
        self.om.add_groups("test2", ["green", "blue", "yellow"])
        self.om.add_groups("test3", ["bleeding", "top"])

        self.om["test1"].properties["id_test"] = "one"
        self.om["test2"].properties["id_test"] = "two"
        self.om["test3"].properties["id_test"] = "three"

        items = self.om.collect_group(
            "top", lambda ob: ob.get_property("id_test"))

        self.assertEqual(items, [("test1", "one"), ("test3", "three")])

    def test_collect_group2(self):
        """
        Test collecting data from an unknown group.
        """

        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        self.om.add_groups("test1", ["foo", "bar", "top"])
        self.om.add_groups("test2", ["green", "blue", "yellow"])
        self.om.add_groups("test3", ["bleeding", "top"])

        self.om["test1"].properties["id_test"] = "one"
        self.om["test2"].properties["id_test"] = "two"
        self.om["test3"].properties["id_test"] = "three"

        items = self.om.collect_group(
            "fop", lambda ob: ob.get_property("id_test"))

        self.assertEqual(items, [])

    def test_set_property1(self):
        """
        Test setting a property for an object.
        """

        self.om.add(TestObject("test1"))
        self.assertEqual(len(self.om["test1"].properties), 2)

        self.om.set_property("test1", "monkey", 12)
        self.assertObjectProperty("test1", "monkey", 12)

        self.om.set_property("test1", "monkey", 25)
        self.assertObjectProperty("test1", "monkey", 25)

    def test_set_property2(self):
        """
        Test setting a property for an unknown object.
        """

        self.om.add(TestObject("test1"))

        with self.assertRaises(KeyError):
            self.om.set_property("test2", "monkey", 12)

    def test_set_property3(self):
        """
        Test setting multiple properties for an object.
        """

        self.om.add(TestObject("test1"))
        self.om.set_properties("test1", [("health", 100), ("mana", 200), ("speed", 10)])
        res = self.om.get_property("test1", "health")
        self.assertEqual(res, 100)
        res = self.om.get_property("test1", "mana")
        self.assertEqual(res, 200)
        res = self.om.get_property("test1", "speed")
        self.assertEqual(res, 10)

    def test_set_property4(self):
        """
        Test setting multiple properties for a group.
        """

        o = TestObject("test1")
        o.add_group("g1")
        self.om.add(o)

        o = TestObject("test2")
        o.add_group("g1")
        self.om.add(o)

        o = TestObject("test3")
        o.add_group("g2")
        self.om.add(o)

        self.om.set_properties_group("g1", [("health", 100), ("mana", 200), ("speed", 10)])
        self.assertEqual(self.om.get_property("test1", "health"), 100)
        self.assertEqual(self.om.get_property("test1", "mana"), 200)
        self.assertEqual(self.om.get_property("test1", "speed"), 10)

        self.assertEqual(self.om.get_property("test2", "health"), 100)
        self.assertEqual(self.om.get_property("test2", "mana"), 200)
        self.assertEqual(self.om.get_property("test2", "speed"), 10)

        self.assertFalse(self.om.has_property("test3", "health"), False)
        self.assertFalse(self.om.has_property("test3", "mana"), False)
        self.assertFalse(self.om.has_property("test3", "speed"), False)

    def test_get_property1(self):
        """
        Test getting a property from an object.
        """

        self.om.add(TestObject("test1"))
        self.assertEqual(len(self.om["test1"].properties), 2)

        self.om.set_property("test1", "monkey", 12)
        self.assertEqual(self.om.get_property("test1", "monkey"), 12)

        self.om.set_property("test1", "monkey", 25)
        self.assertEqual(self.om.get_property("test1", "monkey"), 25)

    def test_get_property2(self):
        """
        Test getting a property from an unknown object.
        """

        self.om.add(TestObject("test1"))

        with self.assertRaises(KeyError):
            self.om.get_property("test2", "monkey")

    def test_get_property3(self):
        """
        Test getting an unknown property from an object.
        """

        self.om.add(TestObject("test1"))

        with self.assertRaises(KeyError):
            self.om.get_property("test1", "monkey")

    def test_remove_property1(self):
        """
        Test removing a property from an object.
        """

        self.om.add(TestObject("test1"))
        self.om.set_property("test1", "jump_height", 20)
        self.om.set_property("test1", "fire_rate", 12)

        self.om.remove_property("test1", "fire_rate")
        self.assertTrue(self.om.has_property("test1", "jump_height"))
        self.assertFalse(self.om.has_property("test1", "fire_rate"))

    def test_remove_property2(self):
        """
        Test removing an unknown property from an object.
        """

        self.om.add(TestObject("test1"))

        with self.assertRaises(KeyError):
            self.om.remove_property("test1", "fire_rate")

    def test_remove_property3(self):
        """
        Test removing a property from a group of objects.
        """

        o = TestObject("test1")
        o.add_group("alpha")
        o.set_property("speed", 50)
        o.set_property("health", 200)
        self.om.add(o)

        o = TestObject("test2")
        o.add_group("alpha")
        o.set_property("speed", 70)
        o.set_property("health", 150)
        self.om.add(o)

        o = TestObject("test3")
        o.set_property("speed", 120)
        o.set_property("health", 110)
        self.om.add(o)

        self.om.remove_property_group("alpha", "speed")
        self.assertFalse(self.om.has_property("test1", "speed"))
        self.assertFalse(self.om.has_property("test2", "speed"))
        self.assertTrue(self.om.has_property("test3", "speed"))

    def test_has_property1(self):
        """
        Test checking a property for an object.
        """

        self.om.add(TestObject("test1"))
        self.assertEqual(len(self.om["test1"].properties), 2)

        self.om.set_property("test1", "monkey", 12)
        self.assertTrue(self.om.has_property("test1", "monkey"))

        self.assertFalse(self.om.has_property("test1", "spider"))

    def test_has_property2(self):
        """
        Test checking a property for an unknown object.
        """

        self.om.add(TestObject("test1"))

        with self.assertRaises(KeyError):
            self.om.has_property("test2", "monkey")

    def test_get_property_default(self):
        """
        Test getting a default value for a unknown property.
        """

        self.om.add(TestObject("test1"))
        self.om.set_property("test1", "fuel", 100)

        self.assertEqual(self.om["test1"].get_property_default("fuel", 200), 100)
        self.assertEqual(self.om["test1"].get_property_default("ammo", 33), 33)

    def test_clear_properties(self):
        """
        Test removing all properties from a given object.
        """

        self.om.add(TestObject("test1"))
        self.om.set_property("test1", "fuel", 100)
        self.om.set_property("test1", "ammo", 33)

        self.om.clear_properties("test1")

        self.assertFalse(self.om.has_property("test1", "fuel"))
        self.assertFalse(self.om.has_property("test1", "ammo"))

    def test_clear_properties_group(self):
        """
        Test removing all the properties from all the objects of a given group.
        """

        self.om.add(TestObject("test1"))
        self.om.set_property("test1", "fuel", 100)
        self.om.set_property("test1", "ammo", 33)

        self.om.add(TestObject("test2"))
        self.om.set_property("test2", "fuel", 200)
        self.om.set_property("test2", "ammo", 70)

        self.om.add(TestObject("test3"))
        self.om.set_property("test3", "fuel", 400)
        self.om.set_property("test3", "ammo", 50)

        self.om.add_group("test1", "enemy")
        self.om.add_group("test2", "player")
        self.om.add_group("test3", "enemy")

        self.om.clear_properties_group("enemy")

        self.assertFalse(self.om.has_property("test1", "fuel"))
        self.assertFalse(self.om.has_property("test1", "ammo"))

        self.assertTrue(self.om.has_property("test2", "fuel"))
        self.assertTrue(self.om.has_property("test2", "ammo"))

        self.assertFalse(self.om.has_property("test3", "fuel"))
        self.assertFalse(self.om.has_property("test3", "ammo"))

    def test_object_send_message1(self):
        """
        Test sending messages to an object.
        """

        o = TestObject("test1")

        o.send_message(GMMessage.single("", "set_pos", (3.5, -7.2)))
        self.assertAlmostEqual(o.pos.x, 3.5)
        self.assertAlmostEqual(o.pos.y, -7.2)

        o.send_message(GMMessage.single("", "set_active", True))
        self.assertTrue(o.active)

        o.send_message(GMMessage.single("", "set_active", False))
        self.assertFalse(o.active)

        o.send_message(GMMessage.single("", "toggle_active", None))
        self.assertTrue(o.active)
        o.send_message(GMMessage.single("", "toggle_active", None))
        self.assertFalse(o.active)

        o.send_message(GMMessage.single("", "set_visible", True))
        self.assertTrue(o.visible)

        o.send_message(GMMessage.single("", "set_visible", False))
        self.assertFalse(o.visible)

        o.send_message(GMMessage.single("", "toggle_visible", None))
        self.assertTrue(o.visible)
        o.send_message(GMMessage.single("", "toggle_visible", None))
        self.assertFalse(o.visible)

        o.send_message(GMMessage.single("", "set_draw_order", 3))
        self.assertEqual(o.draw_order, 3)

        o.send_message(GMMessage.single("", "set_update_order", 5))
        self.assertEqual(o.update_order, 5)

    def test_object_send_message2(self):
        """
        Test sending messages to an object.
        """

        o = TestObject("test1")

        o.send_message(GMMessage.single("", "add_group", "foo"))
        self.assertIn("foo", o.groups)

        res = o.send_message(GMMessage.single("", "in_group", "foo"))
        self.assertTrue(res)

        o.send_message(GMMessage.single("", "remove_group", "foo"))
        self.assertNotIn("foo", o.groups)

        o.send_message(GMMessage.single("", "add_groups", ["g1", "g2", "g3"]))
        self.assertIn("g1", o.groups)
        self.assertIn("g2", o.groups)
        self.assertIn("g3", o.groups)
        o.clear_groups()

        o.add_groups(["foo1", "foo2", "foo3"])
        self.assertIn("foo1", o.groups)
        self.assertIn("foo2", o.groups)
        self.assertIn("foo3", o.groups)
        o.send_message(GMMessage.single("", "clear_groups", None))
        self.assertNotIn("foo1", o.groups)
        self.assertNotIn("foo2", o.groups)
        self.assertNotIn("foo3", o.groups)

        o.send_message(GMMessage.single("", "set_property", ("angry", True)))
        self.assertTrue(o.get_property("angry"))

        res = o.send_message(GMMessage.single("", "get_property", "angry"))
        self.assertTrue(res)

        o.set_property("health", 500)
        o.set_property("speed", 10)

        o.send_message(GMMessage.single("", "remove_property", "speed"))
        self.assertTrue(o.has_property("angry"))
        self.assertTrue(o.has_property("health"))
        self.assertFalse(o.has_property("speed"))

        res = o.send_message(GMMessage.single("", "get_property_default", ("fingers", 5)))
        self.assertEqual(res, 5)

        res = o.send_message(GMMessage.single("", "has_property", "angry"))
        self.assertTrue(res)

        o.send_message(GMMessage.single("", "clear_properties", None))
        self.assertFalse(o.has_property("angry"))

        o.send_message(GMMessage.single("", "set_properties", [("angry", True), ("health", 500), ("speed", 10)]))
        self.assertTrue(o.get_property("angry"))
        self.assertEqual(o.get_property("health"), 500)
        self.assertEqual(o.get_property("speed"), 10)

#    def test_(self):
#        """
#        Test
#        """
#        pass


if __name__ == "__main__":
    unittest.main()

