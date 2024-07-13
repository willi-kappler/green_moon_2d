# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest
import time

from green_moon_2d.gm_animation import GMAnimType, GMAnimation


class TestAnimation(unittest.TestCase):
    def test_empty_frames(self):
        """
        Test for exception if user provides empty frames.
        """

        with self.assertRaises(AssertionError):
            a1 = GMAnimation([])
            a1.current_frame = 0

    def test_change_type(self):
        """
        Test changing the type of the animation.
        """

        frames = [(1, 100), (2, 100), (3, 200)]
        last_frame = len(frames) - 1
        a1 = GMAnimation(frames)

        self.assertEqual(a1.anim_type, GMAnimType.FORWARD)

        a1.current_frame = 1
        a1.change_type(GMAnimType.FORWARD)
        self.assertEqual(a1.current_frame, 0)

        a1.current_frame = 1
        a1.change_type(GMAnimType.BACKWARD)
        self.assertEqual(a1.current_frame, last_frame)

        a1.current_frame = 1
        a1.change_type(GMAnimType.FORWARD_LOOP)
        self.assertEqual(a1.current_frame, 0)

        a1.current_frame = 1
        a1.change_type(GMAnimType.BACKWARD_LOOP)
        self.assertEqual(a1.current_frame, last_frame)

        a1.current_frame = 1
        a1.change_type(GMAnimType.PINGPONG_F)
        self.assertEqual(a1.current_frame, 0)

        a1.current_frame = 1
        a1.change_type(GMAnimType.PINGPONG_B)
        self.assertEqual(a1.current_frame, last_frame)

    def test_get_frame_index(self):
        """
        Test returning the correct frame index.
        """

        a1 = GMAnimation([(5, 100), (10, 100), (8, 300)])
        self.assertEqual(a1.get_frame_index(), 5)

        a1.current_frame = 1
        self.assertEqual(a1.get_frame_index(), 10)

        a1.current_frame = 2
        self.assertEqual(a1.get_frame_index(), 8)

    def test_set_timer_duration(self):
        """
        Test setting the correct timer duration.
        """

        a1 = GMAnimation([(5, 100), (10, 550), (8, 310)])
        a1.set_timer_duration()
        self.assertEqual(a1.timer.duration, 100)

        a1.current_frame = 1
        a1.set_timer_duration()
        self.assertEqual(a1.timer.duration, 550)

        a1.current_frame = 2
        a1.set_timer_duration()
        self.assertEqual(a1.timer.duration, 310)

    def test_update1(self):
        """
        Test updating the animation with FORWARD type.
        """

        a1 = GMAnimation([(2, 100), (4, 200), (7, 100), (1, 200)])
        a1.update()
        self.assertEqual(a1.current_frame, 0)
        self.assertEqual(a1.get_frame_index(), 2)

        time.sleep(0.11)
        a1.update()
        self.assertEqual(a1.current_frame, 1)
        self.assertEqual(a1.get_frame_index(), 4)

        time.sleep(0.11)
        a1.update()
        self.assertEqual(a1.current_frame, 1)
        self.assertEqual(a1.get_frame_index(), 4)

        time.sleep(0.10)
        a1.update()
        self.assertEqual(a1.current_frame, 2)
        self.assertEqual(a1.get_frame_index(), 7)

        time.sleep(0.11)
        a1.update()
        self.assertEqual(a1.current_frame, 3)
        self.assertEqual(a1.get_frame_index(), 1)

        time.sleep(0.21)
        a1.update()
        self.assertEqual(a1.current_frame, 3)
        self.assertEqual(a1.get_frame_index(), 1)
        self.assertEqual(a1.active, False)


if __name__ == "__main__":
    unittest.main()

