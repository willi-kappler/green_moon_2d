# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

from typing import override

import green_moon_2d.gm_engine as gme
from green_moon_2d.gm_scene import GMScene
from green_moon_2d.gm_text import GMText, GMTextEffect1
from green_moon_2d.gm_math import GMAlignment
from green_moon_2d.gm_object import GMObjectManager

import logging
logger = logging.getLogger(__name__)


class TestScene(GMScene):
    def __init__(self):
        super().__init__("TestScene3")

    @override
    def enter(self):
        if not self.initialized:
            resources = gme.GMGlobalResources
            font = resources.get_font("font_cuddly")

            center = GMAlignment.TOP_CENTER

            self.om = GMObjectManager()

            self.om.add(GMText("text1", "TEXT 3", (512.0, 100.0), font, center))
            self.om.add(GMText("text2", "PRESS 1 - 4", (512.0, 200.0), font, center))
            self.om.add(GMText("text3", "TO TOGGLE THE EFFECT", (512.0, 250.0), font, center))
            self.om.add(GMText("text4", "H TO TOGGLE ORIENTATION", (512.0, 300.0), font, center))

            self.text_effect = GMTextEffect1(
                "text5", ">-- DEMO ... TEXT --<", (512.0, 400.0),
                font, GMAlignment.MID_CENTER)
            self.text_effect.effect_sine = True
            self.om.add(self.text_effect)

            # Draw text5 after (= on top of) all other texts since it may overlap.
            self.text_effect.draw_order = 1
            self.om.sort_draw()

            self.initialized = True

    @override
    def update(self):
        self.om.update()

        keys_up = gme.GMGlobalContext.keys_up
        keys_down = gme.GMGlobalContext.keys_down

        if "ESC" in keys_down:
            gme.GMGlobalContext.quit_game = True

        if "1" in keys_down:
            self.text_effect.toggle_sine()

        if "2" in keys_down:
            self.text_effect.toggle_rotate()

        if "3" in keys_down:
            self.text_effect.toggle_scale()

        if "4" in keys_down:
            self.text_effect.toggle_jitter()

        if "H" in keys_up:
            self.text_effect.toggle_orientation()

    @override
    def draw(self):
        self.om.draw()


def main():
    logging.basicConfig(filename="example.log", level=logging.DEBUG)
    logging.info("This is the example text3.")

    engine = gme.GMEngine("config.json", "TestScene3")

    scene = TestScene()
    gme.GMGlobalScenes.add_scene(scene)
    engine.run()


if __name__ == "__main__":
    main()

