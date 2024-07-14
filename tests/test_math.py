# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest

import green_moon_2d.gm_math as gmath


class TestVec2D(unittest.TestCase):
    def test_add1(self):
        """
        Test adding adding a vector (or tuple) to another vector.
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

    def test_add2(self):
        """
        Test adding a vector (or tuple) by mutating the first one.
        """

        v1 = gmath.GMVec2D(2.0, 4.0)
        v2 = gmath.GMVec2D(1.0, -5.0)
        v1.add2(v2)
        self.assertEqual(v1, gmath.GMVec2D(3.0, -1.0))

        v1 = gmath.GMVec2D(2.0, 4.0)
        v3 = (1.0, -5.0)
        v1.add2(v3)
        self.assertEqual(v1, gmath.GMVec2D(3.0, -1.0))


class TestCircle(unittest.TestCase):
    def assertTupleClose(self, t1: tuple[float, float], t2: tuple[float, float]):
        self.assertAlmostEqual(t1[0], t2[0])
        self.assertAlmostEqual(t1[1], t2[1])

    def test_add1(self):
        """
        Test adding a vector (or tuple) to a circle.
        """

        c1 = gmath.GMCircle(1.0, 2.5, 2.5)
        v1 = gmath.GMVec2D(2.8, 0.5)
        c2 = c1 + v1
        self.assertEqual(c2, gmath.GMCircle(3.8, 3.0, 2.5))

        v3 = (2.8, 0.5)
        c2 = c1 + v3
        self.assertEqual(c2, gmath.GMCircle(3.8, 3.0, 2.5))

    def test_add2(self):
        """
        Test adding a vector (or tuple) to a circle by mutating it.
        """

        c1 = gmath.GMCircle(1.0, 2.5, 2.5)
        v1 = gmath.GMVec2D(2.8, 0.5)
        c1.add2(v1)
        self.assertEqual(c1, gmath.GMCircle(3.8, 3.0, 2.5))

        c1 = gmath.GMCircle(1.0, 2.5, 2.5)
        v1 = gmath.GMVec2D(2.8, 0.5)
        c1.add2(v1)
        self.assertEqual(c1, gmath.GMCircle(3.8, 3.0, 2.5))

    def test_inside(self):
        """
        Tests if a point (vector or tuple) is inside a circle.
        """

        c1 = gmath.GMCircle(1.0, 2.5, 2.5)
        v1 = gmath.GMVec2D(1.5, 3.0)
        v2 = gmath.GMVec2D(0.0, 10.0)
        t1 = (0.5, 2.0)

        self.assertEqual(c1.inside(v1), True)
        self.assertEqual(c1.inside(v2), False)
        self.assertEqual(c1.inside(t1), True)

    def test_orbit(self):
        """
        Tests a point orbiting a circle.
        """

        c1 = gmath.GMCircle(5.0, 5.0, 2.0)

        self.assertEqual(c1.orbitCircle(0.0), gmath.GMVec2D(7.0, 5.0))
        self.assertEqual(c1.orbitCircle(90.0), gmath.GMVec2D(5.0, 7.0))
        self.assertEqual(c1.orbitCircle(180.0), gmath.GMVec2D(3.0, 5.0))
        self.assertEqual(c1.orbitCircle(270.0), gmath.GMVec2D(5.0, 3.0))
        self.assertEqual(c1.orbitCircle(360.0), gmath.GMVec2D(7.0, 5.0))
        self.assertEqual(c1.orbitCircle(-90.0), gmath.GMVec2D(5.0, 3.0))
        self.assertEqual(c1.orbitCircle(-180.0), gmath.GMVec2D(3.0, 5.0))
        self.assertEqual(c1.orbitCircle(-270.0), gmath.GMVec2D(5.0, 7.0))
        self.assertEqual(c1.orbitCircle(-360.0), gmath.GMVec2D(7.0, 5.0))

        self.assertTupleClose(c1.orbitTuple(0.0), (7.0, 5.0))
        self.assertTupleClose(c1.orbitTuple(90.0), (5.0, 7.0))
        self.assertTupleClose(c1.orbitTuple(180.0), (3.0, 5.0))
        self.assertTupleClose(c1.orbitTuple(270.0), (5.0, 3.0))
        self.assertTupleClose(c1.orbitTuple(360.0), (7.0, 5.0))
        self.assertTupleClose(c1.orbitTuple(-90.0), (5.0, 3.0))
        self.assertTupleClose(c1.orbitTuple(-180.0), (3.0, 5.0))
        self.assertTupleClose(c1.orbitTuple(-270.0), (5.0, 7.0))
        self.assertTupleClose(c1.orbitTuple(-360.0), (7.0, 5.0))


if __name__ == "__main__":
    unittest.main()

