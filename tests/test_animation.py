# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest

from green_moon_2d.gm_animation import GMAnimType, GMAnimation


class TestAnimation(unittest.TestCase):
    def test_change_type(self):
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

if __name__ == "__main__":
    unittest.main()

