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

            self.text1 = GMText("text1", "PRESS 1 2 3", (512.0, 200.0), self.font1)
            self.text2 = GMText("text2", "TO CHANGE FONT", (512.0, 300.0), self.font1)
            self.text3 = GMText("text3", "THIS IS FONT 1", (512.0, 350.0), self.font1)

            self.initialized = True

    @override
    def update(self):
        if gme.GMGlobalContext.keys["ESC"] == True:
            gme.GMGlobalContext.quit_game = True

    @override
    def draw(self):
        self.text1.draw()
        self.text2.draw()
        self.text3.draw()

def main():
    engine = gme.GMEngine("config.json", "TestScene1")

    scene1 = TestScene()
    gme.GMGlobalScenes.add_scene(scene1)
    engine.run()

if __name__ == "__main__":
    main()
