# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import time


class GMTimer:
    def __init__(self, duration: int):
        self.duration: int = duration
        self.start_time: int = int(time.time() * 1000.0)
        self.active = True

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


