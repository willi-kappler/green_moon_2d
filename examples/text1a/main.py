# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import override

import green_moon_2d.gm_engine as gme
from green_moon_2d.gm_scene import GMScene
from green_moon_2d.gm_text import GMText
from green_moon_2d.gm_math import GMAlignment

import logging
logger = logging.getLogger(__name__)


class ExampleScene(GMScene):
    def __init__(self):
        super().__init__("ExampleScene")

    @override
    def enter(self):
        if not self.initialized:
            resources = gme.GMGlobalResources
            self.font1 = resources.get_font("font_cuddly")
            self.font2 = resources.get_font("font_blagger")
            self.font3 = resources.get_font("font_bbc")

            center = GMAlignment.TOP_CENTER

            self.text1 = GMText("text1", "TEXT 1A", (512.0, 100.0), self.font1, center)
            self.text2 = GMText("text2", "PRESS 1 2 OR 3", (512.0, 200.0), self.font1, center)
            self.text3 = GMText("text3", "TO CHANGE FONT", (512.0, 300.0), self.font1, center)
            self.text4 = GMText("text4", "THIS IS FONT 1", (512.0, 350.0), self.font1, center)

            self.initialized = True

    @override
    def update(self, dt: float):
        keys_down = gme.GMGlobalContext.keys_down

        if "ESC" in keys_down:
            gme.GMGlobalContext.quit_game = True

        if "1" in keys_down:
            self.text1.set_font(self.font1)
            self.text2.set_font(self.font1)
            self.text3.set_font(self.font1)
            self.text4.set_font(self.font1)
            self.text4.set_text("THIS IS FONT 1")

        if "2" in keys_down:
            self.text1.set_font(self.font2)
            self.text2.set_font(self.font2)
            self.text3.set_font(self.font2)
            self.text4.set_font(self.font2)
            self.text4.set_text("THIS IS FONT 2")

        if "3" in keys_down:
            self.text1.set_font(self.font3)
            self.text2.set_font(self.font3)
            self.text3.set_font(self.font3)
            self.text4.set_font(self.font3)
            self.text4.set_text("THIS IS FONT 3")

    @override
    def draw(self):
        self.text1.draw()
        self.text2.draw()
        self.text3.draw()
        self.text4.draw()


def main():
    logging.basicConfig(filename="example.log", level=logging.DEBUG)
    logging.info("This is the example text1a.")

    engine = gme.GMEngine("config.json", "ExampleScene")

    scene = ExampleScene()
    gme.GMGlobalScenes.add_scene(scene)
    engine.run()


if __name__ == "__main__":
    main()
