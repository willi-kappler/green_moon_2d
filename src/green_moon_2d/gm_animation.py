# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from enum import Enum, auto

from green_moon_2d.gm_timer import GMTimer


class GMAnimType(Enum):
    FORWARD = auto()
    BACKWARD = auto()
    FORWARD_LOOP = auto()
    BACKWARD_LOOP = auto()
    PINGPONG_F = auto()
    PINGPONG_B = auto()


class GMAnimation:
    def __init__(self, frames: list[tuple[int, int]]):
        self.current_frame: int = 0
        self.anim_type: GMAnimType = GMAnimType.FORWARD
        self.frames: list[tuple[int, int]] = frames
        assert (len(self.frames) > 0)

        self.timer: GMTimer = GMTimer(self.frames[0][1])
        self.active: bool = True

    def update(self) -> None:
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
                    self.set_timer_duration()
                    self.anim_type = GMAnimType.PINGPONG_B
                case GMAnimType.PINGPONG_B:
                    self.current_frame -= 1
                    if self.current_frame < 0:
                        self.current_frame = 1
                    self.set_timer_duration()
                    self.anim_type = GMAnimType.PINGPONG_F

    def change_type(self, new_type: GMAnimType) -> None:
        """
        Change the type of this animation.
        The current frame will be changed accordingly.
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

    def get_frame_index(self) -> int:
        """
        Returns the current frame index of the animation.
        """
        return self.frames[self.current_frame][0]

    def set_timer_duration(self) -> None:
        """
        Sets the new duration and restarts the timer.
        """
        new_duration = self.frames[self.current_frame][1]
        self.timer.set_duration_restart(new_duration)

