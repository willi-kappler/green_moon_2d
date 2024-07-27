
from typing import override

import green_moon_2d.gm_engine as gme
from green_moon_2d.gm_scene import GMScene
from green_moon_2d.gm_text import GMText

class TestScene(GMScene):
    def __init__(self):
        super().__init__("TestScene1")

    def enter(self):
        if not self.initialized:
            resources = gme.GMGlobalResources
            self.font1 = resources.get_font("font_cuddly")
            self.font2 = resources.get_font("font_blagger")
            self.font3 = resources.get_font("font_bbc")

            self.text1 = GMText("text1", "", (0.0, 0.0), self.font1)
            self.text2 = GMText("text2", "", (0.0, 0.0), self.font1)
            self.text3 = GMText("text3", "", (0.0, 0.0), self.font1)

            self.initialized = True

    def update(self):
        pass

    def draw(self):
        pass

def main():
    engine = gme.GMEngine("config.json", "TestScene1")

    scene1 = TestScene()
    gme.GMGlobalScenes.add_scene(scene1)
    engine.run()

if __name__ == "__main__":
    main()
