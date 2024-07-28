# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import override
import sys

sys.path.append("../../src/")

import green_moon_2d.gm_engine as gme
from green_moon_2d.gm_scene import GMScene
from green_moon_2d.gm_text import GMText
from green_moon_2d.gm_math import GMAlignment
from green_moon_2d.gm_object import GMObjectManager


class TestScene(GMScene):
    def __init__(self):
        super().__init__("TestScene2")

    @override
    def enter(self):
        if not self.initialized:
            resources = gme.GMGlobalResources
            font = resources.get_font("font_cuddly")

            center = GMAlignment.TOP_CENTER

            self.om = GMObjectManager()

            self.om.add(GMText("text1", "TEXT 2", (512.0, 100.0), font, center))
            self.om.add(GMText("text2", "PRESS 1 - 9", (512.0, 200.0), font, center))
            self.om.add(GMText("text3", "TO CHANGE ALIGNMENT", (512.0, 300.0), font, center))
            self.om.add(GMText("text4", "PRESS H FOR ORIENTATION", (512.0, 350.0), font, center))
            self.om.add(GMText("text5", "DEMO TEXT", (512.0, 450.0), font, center))

            # Draw text5 after (= on top of) all other texts since it may overlap.
            self.om["text5"].draw_order = 1
            self.om.sort_draw()

            self.initialized = True

    @override
    def update(self):
        if gme.GMGlobalContext.keys["ESC"]:
            gme.GMGlobalContext.quit_game = True

        if gme.GMGlobalContext.keys["1"]:
            self.om.send_message("text5", ("set_alignment", GMAlignment.TOP_LEFT))

        if gme.GMGlobalContext.keys["2"]:
            self.om.send_message("text5", ("set_alignment", GMAlignment.TOP_CENTER))

        if gme.GMGlobalContext.keys["3"]:
            self.om.send_message("text5", ("set_alignment", GMAlignment.TOP_RIGHT))

    @override
    def draw(self):
        self.om.draw()


def main():
    engine = gme.GMEngine("config.json", "TestScene1")

    scene1 = TestScene()
    gme.GMGlobalScenes.add_scene(scene1)
    engine.run()


if __name__ == "__main__":
    main()

