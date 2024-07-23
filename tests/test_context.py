# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest

from green_moon_2d.gm_context import GMContext


class TestContext(unittest.TestCase):
    def test_set_property(self):
        """
        Test setting a custom property.
        """

        ctx = GMContext()
        ctx.set_property("points", 123)
        self.assertEqual(ctx.game_property["points"], 123)

    def test_get_property1(self):
        """
        Test getting a custom property.
        """

        ctx = GMContext()
        ctx.set_property("points", 123)
        self.assertEqual(ctx.get_property("points"), 123)

    def test_get_property2(self):
        """
        Test getting a custom unknown property.
        """

        ctx = GMContext()
        ctx.set_property("points", 123)

        with self.assertRaises(KeyError):
            self.assertEqual(ctx.get_property("coins"), 123)

    def test_has_property(self):
        """
        Test checking if a custom property has been set.
        """

        ctx = GMContext()
        ctx.set_property("level", 22)
        self.assertTrue(ctx.has_property("level"))
        self.assertFalse(ctx.has_property("points"))

    # The rest of the methods are tested in test_scene.py!


if __name__ == "__main__":
    unittest.main()


