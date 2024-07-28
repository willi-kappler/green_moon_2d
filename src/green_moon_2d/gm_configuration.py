# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import json


class GMConfiguration:
    def __init__(self):
        self.config_file: str = "config.json"
        self.fps: int = 60
        self.fullscreen: bool = False
        self.resource_file: str = "resources.json"
        self.screen_height: int = 600
        self.screen_width: int = 800
        self.window_title: str = "Made with GreenMoon2D"

    def load_config(self, file_name: str):
        """
        Load the configuration (JSON format) from the given file name.

        :param file_name: File name of the configuration.
        """

        with open(file_name, "r") as f:
            data = json.load(f)

        if "config_file" in data:
            self.config_file = data["config_file"]

        if "fps" in data:
            self.fps = data["fps"]

        if "fullscreen" in data:
            self.fullscreen = data["fullscreen"]

        if "resource_file" in data:
            self.resource_file = data["resource_file"]

        if "screen_height" in data:
            self.screen_height = data["screen_height"]

        if "screen_width" in data:
            self.screen_width = data["screen_width"]

        if "window_title" in data:
            self.window_title = data["window_title"]

        self.config_file = file_name

    def save_config(self):
        """
        Saves the game configuration.
        """

        with open(self.config_file, "w") as f:
            json.dump(self.__dict__, f, indent=2, sort_keys=True)


