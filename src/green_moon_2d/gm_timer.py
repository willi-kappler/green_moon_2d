# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import time


class GMTimer:
    """
    This class defines a timer that is set with a duration in ms.
    """

    def __init__(self, duration: int):
        self.duration: int = duration
        self.start_time: int = int(time.time() * 1000.0)
        self.active = True

    def finished(self) -> bool:
        """
        When the timer is active, checks if the timer is finished.

        :return: True if finished otherwise False. Also False if the timer is inactive.
        :rtype: bool
        """
        if self.active:
            current_time = int(time.time() * 1000.0)
            return (current_time - self.start_time) > self.duration
        else:
            return False

    def set_duration(self, duration: int) -> None:
        """
        Sets the new duration for this timer. Note that it is not restarted automatically.

        :param int duration: The new duration in ms.
        """
        self.duration = duration

    def set_duration_restart(self, duration: int) -> None:
        """
        Sets the new duration and restarts the timer.

        :param int duration: The new duration in ms.
        """
        self.duration = duration
        self.restart()

    def restart(self) -> None:
        """
        Restarts the timer and set it to active.
        """
        self.start_time = int(time.time() * 1000.0)
        self.active = True


