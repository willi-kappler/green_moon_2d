# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import pygame

from green_moon_2d.gm_configuration import GMConfiguration
from green_moon_2d.gm_context import GMContext
from green_moon_2d.gm_scene import GMScene, GMSceneManager


class GMEngine:
    def __init__(self, config_file: str = ""):
        self.context: GMContext = GMContext()
        self.scene_manager = GMSceneManager()

        if len(config_file) > 0:
            self.context.load_config(config_file)

    def load_config(self, config_file: str):
        """
        Loads the configuration from the given file name.

        :param config_file: The name of the config file to load.
        """
        self.context.load_config(config_file)

    def set_config(self, config: GMConfiguration):
        """
        Sets the configuration directly.

        :param config: The new configuration.
        """
        self.context.config = config

    def add_scene(self, scene: GMScene):
        """
        Adds a new scene to the scene manager.

        :param scene: The new scene to be added.
        """
        self.scene_manager.add_scene(scene)

    def run(self):
        """
        Starts the engine and runs the game loop.
        """

        config: GMConfiguration = self.context.config
        ctx: GMContext = self.context
        smgr: GMSceneManager = self.scene_manager

        pygame.init()
        flags = pygame.FULLSCREEN if config.fullscreen else 0
        ctx.screen = pygame.display.set_mode((config.screen_width, config.screen_height), flags)
        clock = pygame.time.Clock()
        pygame.display.set_caption(config.window_title)

        while not ctx.quit_game:
            smgr.update(ctx)
            ctx.clear()
            smgr.draw(ctx)
            pygame.display.flip()
            ctx.dt = clock.tick(config.fps)

        pygame.quit()


