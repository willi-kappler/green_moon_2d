# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest

from typing import Any

from green_moon_2d.gm_object import GMObject, GMObjectManager
from green_moon_2d.gm_context import GMContext


class TestObject(GMObject):
    def __init__(self, name):
        super().__init__(name)

        self.properties["update_called"] = 0
        self.properties["draw_called"] = 0

    def update(self, context: GMContext):
        self.properties["update_called"] += 1

    def draw(self, context: GMContext):
        self.properties["draw_called"] += 1


class TestObjectManager(unittest.TestCase):
    def setUp(self):
        self.om = GMObjectManager()

    def assertNumObjects(self, n: int) -> None:
        self.assertEqual(len(self.om.objects), n)

    def assertObjectIndex(self, name: str, n: int) -> None:
        self.assertEqual(self.om.get_index(name), n)

    def assertObjectGroups(self, name: str, groups: list[str]) -> None:
        self.assertEqual(self.om.get(name).groups, set(groups))

    def assertObjectProperty(self, name: str, property: str, val: Any):
        self.assertEqual(self.om.get(name).properties[property], val)

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

        obj = self.om.get("test1")
        self.assertEqual(obj.name, "test1")
        obj = self.om.get("test2")
        self.assertEqual(obj.name, "test2")
        obj = self.om.get("test3")
        self.assertEqual(obj.name, "test3")

    def test_get2(self):
        """
        Test get object ba name with an unknown name.
        """
        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))
        self.om.add(TestObject("test3"))

        with self.assertRaises(KeyError):
            self.om.get("test4")

    def test_update(self):
        """
        Test calling update on all objects.
        """
        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        context = GMContext()
        self.om.update(context)

        self.assertObjectProperty("test1", "update_called", 1)
        self.assertObjectProperty("test2", "update_called", 1)

        self.om.add(TestObject("test3"))
        self.om.update(context)

        self.assertObjectProperty("test1", "update_called", 2)
        self.assertObjectProperty("test2", "update_called", 2)
        self.assertObjectProperty("test3", "update_called", 1)

    def test_draw(self):
        """
        Test calling draw on all objects.
        """
        self.om.add(TestObject("test1"))
        self.om.add(TestObject("test2"))

        context = GMContext()
        self.om.draw(context)

        self.assertObjectProperty("test1", "draw_called", 1)
        self.assertObjectProperty("test2", "draw_called", 1)

        self.om.add(TestObject("test3"))
        self.om.draw(context)

        self.assertObjectProperty("test1", "draw_called", 2)
        self.assertObjectProperty("test2", "draw_called", 2)
        self.assertObjectProperty("test3", "draw_called", 1)

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

        self.om.add_group("test1", ["bar", "baz"])
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
        self.om.add_group("test1", ["foo", "bar"])
        self.om.add_group("test2", ["green", "blue", "yellow"])

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

        self.om.add_group("test1", ["foo", "bar", "top"])
        self.om.add_group("test2", ["green", "blue", "yellow", "top"])
        self.om.add_group("test3", ["poison"])

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

        self.om.add_group("test1", ["foo", "bar", "top"])
        self.om.add_group("test2", ["green", "blue", "yellow", "top"])

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

        self.om.add_group("test1", ["foo", "bar", "top"])
        self.om.add_group("test2", ["green", "blue", "yellow"])
        self.om.add_group("test3", ["bleeding", "top"])

        self.om.get("test1").properties["Called"] = "No"
        self.om.get("test2").properties["Called"] = "No"
        self.om.get("test3").properties["Called"] = "No"

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

        self.om.add_group("test1", ["foo", "bar", "top"])
        self.om.add_group("test2", ["blue", "top"])

        self.om.get("test1").properties["Called"] = "No"
        self.om.get("test2").properties["Called"] = "No"

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

        self.om.add_group("test1", ["foo", "bar", "top"])
        self.om.add_group("test2", ["green", "blue", "yellow"])
        self.om.add_group("test3", ["bleeding", "top"])

        self.om.get("test1").properties["Called"] = "No"
        self.om.get("test2").properties["Called"] = "No"
        self.om.get("test3").properties["Called"] = "No"

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

        self.om.add_group("test1", ["foo", "bar", "top"])
        self.om.add_group("test2", ["green", "blue", "yellow"])
        self.om.add_group("test3", ["bleeding", "top"])

        self.om.get("test1").properties["Called"] = "No"
        self.om.get("test2").properties["Called"] = "No"
        self.om.get("test3").properties["Called"] = "No"

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
        pass

    def test_collect_group2(self):
        """
        Test collecting data from an unknown group.
        """
        pass

    def test_set_property(self):
        """
        Test
        """
        pass

    def test_get_property(self):
        """
        Test
        """
        pass

    def test_has_property(self):
        """
        Test
        """
        pass


#    def test_(self):
#        """
#        Test
#        """
#        pass


if __name__ == "__main__":
    unittest.main()
