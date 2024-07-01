# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest

from green_moon_2d.gm_object import GMObject, GMObjectManager
from green_moon_2d.gm_context import GMContext

class TestObject(GMObject):
    def __init__(self, name):
        super().__init__(name)

        self.update_called = 0
        self.draw_called = 0

    def update(self, context: GMContext):
        self.update_called += 1

    def draw(self, context: GMContext):
        self.draw_called += 1

class TestObjectManager(unittest.TestCase):
    def setUp(self):
        self.om = GMObjectManager()

    def test_add_object1(self):
        """
        Test
        """
        pass

    def test_add_object2(self):
        """
        Test
        """
        pass

    def test_delete_object1(self):
        """
        Test
        """
        pass

    def test_delete_object2(self):
        """
        Test
        """
        pass

    def test_sort_update(self):
        """
        Test
        """
        pass

    def test_sort_draw(self):
        """
        Test
        """
        pass

    def test_(self):
        """
        Test
        """
        pass

    def test_get_index(self):
        """
        Test
        """
        pass

    def test_get1(self):
        """
        Test
        """
        pass

    def test_get2(self):
        """
        Test
        """
        pass

    def test_update(self):
        """
        Test
        """
        pass

    def test_draw(self):
        """
        Test
        """
        pass

    def test_add_group1(self):
        """
        Test
        """
        pass

    def test_add_group2(self):
        """
        Test
        """
        pass

    def test_remove_group1(self):
        """
        Test
        """
        pass

    def test_remove_group2(self):
        """
        Test
        """
        pass

    def test_clear_groups1(self):
        """
        Test
        """
        pass

    def test_clear_groups2(self):
        """
        Test
        """
        pass

    def test_iter_group1(self):
        """
        Test
        """
        pass

    def test_iter_group2(self):
        """
        Test
        """
        pass

    def test_apply_group1(self):
        """
        Test
        """
        pass

    def test_apply_group2(self):
        """
        Test
        """
        pass

    def test_collect_group1(self):
        """
        Test
        """
        pass

    def test_collect_group2(self):
        """
        Test
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

if __name__ == '__main__':
    unittest.main()

