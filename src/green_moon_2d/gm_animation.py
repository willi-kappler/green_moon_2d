# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any, override
from enum import Enum, auto
import time

from green_moon_2d.gm_context import GMContext
from green_moon_2d.gm_object import GMObject


class GMTimer(GMObject):
    def __init__(self, duration: int):
        self.duration: int = duration
        self.start_time: int = int(time.time() * 1000.0)

    @override
    def update(self, context: GMContext) -> None:
        if self.active:
            raise NotImplementedError
            # TODO:

    @override
    def send_message(self, msg: Any) -> Any:
        raise NotImplementedError
        # TODO:

    def finished(self) -> bool:
        if self.active:
            current_time = int(time.time() * 1000.0)
            return (current_time - self.start_time) > self.duration
        else:
            return False

    def set_duration(self, duration: int) -> None:
        self.duration = duration

    def set_duration_restart(self, duration: int) -> None:
        self.duration = duration
        self.restart()

    def restart(self) -> None:
        self.start_time = int(time.time() * 1000.0)
        self.active = True


class GMAnimType(Enum):
    FORWARD = auto()
    BACKWARD = auto()
    FORWARD_LOOP = auto()
    BACKWARD_LOOP = auto()
    PINGPONG_F = auto()
    PINGPONG_B = auto()


class GMAnimation(GMObject):
    def __init__(self, frames: list[tuple[int, int]]):
        self.current_frame: int = 0
        self.anim_type: GMAnimType = GMAnimType.FORWARD
        self.frames: list[tuple[int, int]] = frames
        assert (len(self.frames) > 0)

        self.timer: GMTimer = GMTimer(self.frames[0][1])

    @override
    def update(self, context: GMContext) -> None:
        if self.active:
            match self.anim_type:
                case GMAnimType.FORWARD:
                    pass
                case GMAnimType.BACKWARD:
                    pass
                case GMAnimType.FORWARD_LOOP:
                    pass
                case GMAnimType.BACKWARD_LOOP:
                    pass
                case GMAnimType.PINGPONG_F:
                    pass
                case GMAnimType.PINGPONG_B:
                    pass

    @override
    def send_message(self, msg: Any) -> Any:
        raise NotImplementedError
        # TODO:

    def change_type(self, new_type: GMAnimType) -> None:
        """
        Change the type of this animation.
        The current frame will be reset to 0.
        """
        self.anim_type = new_type
        self.current_frame = 0

    def get_frame_index(self) -> int:
        return self.frames[self.current_frame][0]

