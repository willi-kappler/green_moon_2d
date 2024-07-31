# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import override

import green_moon_2d.gm_engine as gme
from green_moon_2d.gm_scene import GMScene
from green_moon_2d.gm_object import GMObjectManager
from green_moon_2d.gm_text import GMText
from green_moon_2d.gm_math import GMAlignment
from green_moon_2d.gm_sprite import GMSprite

import logging
logger = logging.getLogger(__name__)


class ExampleScene(GMScene):
    def __init__(self):
        super().__init__("ExampleScene")

    @override
    def enter(self):
        if not self.initialized:
            self.om = GMObjectManager()

            resources = gme.GMGlobalResources
            self.font1 = resources.get_font("font_cuddly")

            center = GMAlignment.TOP_CENTER

            self.om.add(GMText("text1", "SPRITE 1", (512.0, 100.0), self.font1, center))

            self.initialized = True

    @override
    def update(self):
        self.om.update()

        keys_down = gme.GMGlobalContext.keys_down

        if "ESC" in keys_down:
            gme.GMGlobalContext.quit_game = True


    @override
    def draw(self):
        self.om.draw()


def main():
    logging.basicConfig(filename="example.log", level=logging.DEBUG)
    logging.info("This is the example sprite1.")

    engine = gme.GMEngine("config.json", "ExampleScene")

    scene = ExampleScene()
    gme.GMGlobalScenes.add_scene(scene)
    engine.run()


if __name__ == "__main__":
    main()
