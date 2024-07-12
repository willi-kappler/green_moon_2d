# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest
import time

from green_moon_2d.gm_timer import GMTimer


class TestVec2D(unittest.TestCase):
    def test_finished(self):
        """
        Test if timer calculates finished value correctly.
        """

        t1 = GMTimer(100)
        self.assertEqual(t1.finished(), False)

        time.sleep(0.055)
        self.assertEqual(t1.finished(), False)

        time.sleep(0.055)
        self.assertEqual(t1.finished(), True)

        t1.active = False
        self.assertEqual(t1.finished(), False)

        time.sleep(0.11)
        self.assertEqual(t1.finished(), False)

    def test_set_duration(self):
        """
        Test changing the duration of the timer.
        """

        t1 = GMTimer(200)

        t1.set_duration(500)
        self.assertEqual(t1.duration, 500)

    def test_restart(self):
        """
        Test restating the time once it has finished.
        """

        t1 = GMTimer(100)

        time.sleep(0.11)
        self.assertEqual(t1.finished(), True)

        t1.restart()
        self.assertEqual(t1.finished(), False)

        time.sleep(0.11)
        self.assertEqual(t1.finished(), True)

        t1.active = False
        self.assertEqual(t1.finished(), False)

        t1.restart()
        time.sleep(0.11)
        self.assertEqual(t1.finished(), True)

    def test_set_duration_restart(self):
        """
        Test changing the duration and restarting the timer.
        """

        t1 = GMTimer(100)
        t1.active = False

        self.assertEqual(t1.finished(), False)

        time.sleep(0.11)
        self.assertEqual(t1.finished(), False)

        t1.set_duration_restart(200)
        self.assertEqual(t1.active, True)
        self.assertEqual(t1.duration, 200)
        self.assertEqual(t1.finished(), False)

        time.sleep(0.21)
        self.assertEqual(t1.finished(), True)


if __name__ == "__main__":
    unittest.main()

