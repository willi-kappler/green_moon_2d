# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import json


class GMConfiguration:
    def __init__(self):
        self.screen_width: int = 800
        self.screen_height: int = 600
        self.fullscreen: bool = False
        self.window_title: str = "Made with GreenMoon2D"
        self.fps: int = 60
        self.config_file: str = "config.json"
        self.resource_file: str = "resources.json"

    def load_config(self, file_name: str):
        """
        Load the configuration (JSON format) from the given file name.

        :param file_name: File name of the configuration.
        """

        with open(file_name, "r") as f:
            data = json.load(f)

        if "screen_width" in data:
            self.screen_width = data["screen_width"]

        if "screen_height" in data:
            self.screen_height = data["screen_height"]

        if "fullscreen" in data:
            self.fullscreen = data["fullscreen"]

        if "window_title" in data:
            self.window_title = data["window_title"]

        if "fps" in data:
            self.fps = data["fps"]

        if "resource_file" in data:
            self.resource_file = data["resource_file"]

        self.config_file = file_name

    def save_config(self):
        """
        Saves the game configuration.
        """

        with open(self.config_file, "w") as f:
            json.dump(self.__dict__, f, indent=2, sort_keys=True)


