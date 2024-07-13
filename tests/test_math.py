# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest

import green_moon_2d.gm_math as gmath


class TestVec2D(unittest.TestCase):
    def test_add(self):
        """
        Test adding two vectors together.
        """

        v1 = gmath.GMVec2D(1.0, 4.0)
        v2 = gmath.GMVec2D(2.5, -3.5)
        v3 = v1 + v2

        self.assertEqual(v3, gmath.GMVec2D(3.5, 0.5))

        v1 = gmath.GMVec2D(2.7, -5.2)
        v2 = gmath.GMVec2D(0.0, 0.0)
        v3 = v1 + v2

        self.assertEqual(v3, gmath.GMVec2D(2.7, -5.2))

        v1 = gmath.GMVec2D(0.0, 0.0)
        v2 = gmath.GMVec2D(9.9, 7.1)
        v3 = v1 + v2

        self.assertEqual(v3, gmath.GMVec2D(9.9, 7.1))


if __name__ == "__main__":
    unittest.main()

