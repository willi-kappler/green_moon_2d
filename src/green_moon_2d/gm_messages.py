# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines messages used in the game engine.
"""

from typing import Any

import green_moon_2d.gm_interfaces as gmi


class GMSceneMessage:
    def __init__(
            self, kind: str,
            scene: None | gmi.GMSceneInterface = None,
            scene_name: str = "", custom_message: Any = None):
        """
        A message that is send from the current game context to the scene manager.

        :param kind: The kind of the message: "add", "delete", "change", "push", 
            "pop", "update_stack_top", "draw_stack_top", "update_scene", "draw_scene",
            "send_message"
        :param scene: The actual scene or None.
        :param scene_name: The name of the scene or empty.
        :param custom_message: A custom message or None.
        """

        self.kind: str = kind
        self.scene = scene
        self.scene_name: str = scene_name
        self.custom_message: Any = custom_message


