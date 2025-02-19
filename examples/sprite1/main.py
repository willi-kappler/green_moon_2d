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
    def enter(self) -> None:
        if not self.initialized:
            self.om = GMObjectManager()

            resources = gme.GMGlobalResources
            font1 = resources.get_font("font_cuddly")
            self.om.add(GMText("text1", "SPRITE 1", (512.0, 100.0), font1, GMAlignment.TOP_CENTER))

            texture = resources.get_texture("tex_bat1")
            animation = resources.get_animation("anim_bat1")
            self.om.add(GMSprite((512.0, 200.0), "bat1", animation, texture))

            texture = resources.get_texture("tex_ghost1")
            animation = resources.get_animation("anim_ghost1")
            self.om.add(GMSprite((512.0, 250.0), "ghost1", animation, texture))

            texture = resources.get_texture("tex_ice_cream1")
            animation = resources.get_empty_animation()
            self.om.add(GMSprite((512.0, 300.0), "ice1", animation, texture))

            self.initialized = True

    @override
    def update(self, dt: float) -> None:
        self.om.update(dt)

        keys_down = gme.GMGlobalContext.keys_down

        if "ESC" in keys_down:
            gme.GMGlobalContext.quit_game = True


    @override
    def draw(self) -> None:
        gme.GMGlobalContext.clear_screen()
        self.om.draw()


def main() -> None:
    logging.basicConfig(filename="example.log", level=logging.DEBUG)
    logging.info("This is the example sprite1.")

    engine = gme.GMEngine("config.json", "ExampleScene")

    scene = ExampleScene()
    gme.GMGlobalScenes.add_scene(scene)
    engine.run()


if __name__ == "__main__":
    main()
