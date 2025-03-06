# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import Any

import enum


class GMMessageType(enum.Enum):
    SINGLE = 0
    MULTIPLE = 1
    GROUP = 2
    MULTI_GROUP = 3


class GMMessage:
    def __init__(self, msg_type: GMMessageType, target: str | list[str],
            child: str, command: str, value: Any):
        self.msg_type: GMMessageType = msg_type
        self.targets: list[str] = []
        self.child: str = child
        self.command: str = command
        self.value: Any = value

        match target:
            case str():
                self.targets = [target]
            case list():
                self.targets = target
            case _:
                raise ValueError(f"Expected string or list for target, got: {target}")

    @staticmethod
    def empty() -> "GMMessage":
        msg = GMMessage(GMMessageType.SINGLE, "", "", "", None)
        return msg

    @staticmethod
    def single(target: str, command: str, value: Any) -> "GMMessage":
        msg = GMMessage(GMMessageType.SINGLE, target, "", command, value)
        return msg

    @staticmethod
    def single_child(target: str, child: str, command: str, value: Any) -> "GMMessage":
        msg = GMMessage(GMMessageType.SINGLE, target, child, command, value)
        return msg

    @staticmethod
    def multiple(target: list[str], command: str, value: Any) -> "GMMessage":
        msg = GMMessage(GMMessageType.MULTIPLE, target, "", command, value)
        return msg

    @staticmethod
    def multiple_child(target: list[str], child: str, command: str, value: Any) -> "GMMessage":
        msg = GMMessage(GMMessageType.MULTIPLE, target, child, command, value)
        return msg

    @staticmethod
    def group(target: str, command: str, value: Any) -> "GMMessage":
        msg = GMMessage(GMMessageType.GROUP, target, "", command, value)
        return msg

    @staticmethod
    def group_child(target: str, child: str, command: str, value: Any) -> "GMMessage":
        msg = GMMessage(GMMessageType.GROUP, target, child, command, value)
        return msg

    @staticmethod
    def multi_group(target: list[str], command: str, value: Any) -> "GMMessage":
        msg = GMMessage(GMMessageType.MULTI_GROUP, target, "", command, value)
        return msg

    @staticmethod
    def mult_group_child(target: list[str], child: str, command: str, value: Any) -> "GMMessage":
        msg = GMMessage(GMMessageType.MULTI_GROUP, target, child, command, value)
        return msg

