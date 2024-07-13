# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMAnimType enum and the GMAnimation class that is used to animate
objects ( for example sprites).
"""

from enum import Enum, auto

from green_moon_2d.gm_timer import GMTimer


class GMAnimType(Enum):
    """
    This class defins the various animation types.
    FORWARD: Runs the animation forward once.
    BACKWARD: Runs the animation backward once.
    FORWARD_LOOP: Runs the animation forward in a loop.
    BACKWARD_LOOP: Runs the animation backward in a loop.
    PINGPONG_F: Runs the animation forward once and then
        backward once and then repeats.
    """

    FORWARD = auto()
    BACKWARD = auto()
    FORWARD_LOOP = auto()
    BACKWARD_LOOP = auto()
    PINGPONG_F = auto()
    PINGPONG_B = auto()


class GMAnimation:
    """
    This class defines the animation for object like sprites.
    It has a built in timer and uses animation frames to manage indices into the
    sprite sheet. For example: [(2, 200), (3, 200), (4, 250), (5, 400)].
    The first value is the index, the second value is the duration in milli seconds.
    (2, 200) means: index 2 in the sprite sheet and display duration of 200 ms.
    """

    def __init__(self, frames: list[tuple[int, int]]):
        """
        :param frames: A list of tuples containing the index and the duration.
        """
        self.current_frame: int = 0
        self.anim_type: GMAnimType = GMAnimType.FORWARD
        self.frames: list[tuple[int, int]] = frames
        assert (len(self.frames) > 0)

        self.timer: GMTimer = GMTimer(self.frames[0][1])
        self.active: bool = True

    def update(self) -> None:
        """
        This method is called once per frame and checks the timer (duration) and moves on to the next
        frame in the given list of frames.
        Depening on the animation type different actions are performed.
        See description of GMAnimType.
        """
        if self.active and self.timer.finished():
            match self.anim_type:
                case GMAnimType.FORWARD:
                    self.current_frame += 1
                    if self.current_frame >= len(self.frames):
                        self.active = False
                        self.current_frame = len(self.frames) - 1
                    else:
                        self.set_timer_duration()
                case GMAnimType.BACKWARD:
                    self.current_frame -= 1
                    if self.current_frame < 0:
                        self.active = False
                        self.current_frame = 0
                    else:
                        self.set_timer_duration()
                case GMAnimType.FORWARD_LOOP:
                    self.current_frame += 1
                    if self.current_frame >= len(self.frames):
                        self.current_frame = 0
                    self.set_timer_duration()
                case GMAnimType.BACKWARD_LOOP:
                    self.current_frame -= 1
                    if self.current_frame < 0:
                        self.current_frame = len(self.frames) - 1
                    self.set_timer_duration()
                case GMAnimType.PINGPONG_F:
                    self.current_frame += 1
                    if self.current_frame >= len(self.frames):
                        self.current_frame = len(self.frames) - 2
                        self.anim_type = GMAnimType.PINGPONG_B
                    self.set_timer_duration()
                case GMAnimType.PINGPONG_B:
                    self.current_frame -= 1
                    if self.current_frame < 0:
                        self.current_frame = 1
                        self.anim_type = GMAnimType.PINGPONG_F
                    self.set_timer_duration()

    def change_type(self, new_type: GMAnimType) -> None:
        """
        Change the type of this animation.
        The current frame will be changed accordingly.
        The animation is set to active and the timer is restarted with the new duration.

        :param new_type: The new type for this animation.
        """
        self.anim_type = new_type

        match self.anim_type:
            case GMAnimType.FORWARD:
                self.current_frame = 0
            case GMAnimType.BACKWARD:
                self.current_frame = len(self.frames) - 1
            case GMAnimType.FORWARD_LOOP:
                self.current_frame = 0
            case GMAnimType.BACKWARD_LOOP:
                self.current_frame = len(self.frames) - 1
            case GMAnimType.PINGPONG_F:
                self.current_frame = 0
                assert (len(self.frames) > 1)
            case GMAnimType.PINGPONG_B:
                self.current_frame = len(self.frames) - 1
                assert (len(self.frames) > 1)

        self.active = True
        self.set_timer_duration()

    def get_frame_index(self) -> int:
        """
        Returns the current frame index of the animation.

        :return: The current frame index.
        :rtype: int
        """
        return self.frames[self.current_frame][0]

    def set_timer_duration(self) -> None:
        """
        Sets the new duration and restarts the timer.
        """
        new_duration = self.frames[self.current_frame][1]
        self.timer.set_duration_restart(new_duration)

