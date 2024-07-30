# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import override

import green_moon_2d.gm_engine as gme
from green_moon_2d.gm_scene import GMScene
from green_moon_2d.gm_text import GMText
from green_moon_2d.gm_math import GMAlignment
from green_moon_2d.gm_object import GMObjectManager

import logging
logger = logging.getLogger(__name__)


class TestScene(GMScene):
    def __init__(self):
        super().__init__("TestScene1")

    @override
    def enter(self):
        if not self.initialized:
            resources = gme.GMGlobalResources
            self.font1 = resources.get_font("font_cuddly")
            self.font2 = resources.get_font("font_blagger")
            self.font3 = resources.get_font("font_bbc")

            center = GMAlignment.TOP_CENTER

            self.om = GMObjectManager()

            self.om.add(GMText("text1", "TEXT 1B", (512.0, 100.0), self.font1, center))
            self.om.add(GMText("text2", "PRESS 1 2 OR 3", (512.0, 200.0), self.font1, center))
            self.om.add(GMText("text3", "TO CHANGE FONT", (512.0, 300.0), self.font1, center))
            self.font_text = GMText("text4", "THIS IS FONT 1", (512.0, 350.0), self.font1, center)
            self.om.add(self.font_text)

            self.om.add_group("text1", "group1")
            self.om.add_group("text2", "group1")
            self.om.add_group("text3", "group1")
            self.om.add_group("text4", "group1")

            self.initialized = True

    @override
    def update(self):
        keys_down = gme.GMGlobalContext.keys_down

        if "ESC" in keys_down:
            gme.GMGlobalContext.quit_game = True

        if "1" in keys_down:
            self.om.send_message_group("group1", ("set_font", self.font1))
            self.font_text.set_text("THIS IS FONT 1")

        if "2" in keys_down:
            self.om.send_message_group("group1", ("set_font", self.font2))
            self.font_text.set_text("THIS IS FONT 2")

        if "3" in keys_down:
            self.om.send_message_group("group1", ("set_font", self.font3))
            self.font_text.set_text("THIS IS FONT 3")

    @override
    def draw(self):
        self.om.draw()


def main():
    logging.basicConfig(filename="example.log", level=logging.DEBUG)
    logging.info("This is the example text1b.")

    engine = gme.GMEngine("config.json", "TestScene1")

    scene1 = TestScene()
    gme.GMGlobalScenes.add_scene(scene1)
    engine.run()


if __name__ == "__main__":
    main()

