# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import pygame

from green_moon_2d.gm_configuration import GMConfiguration
from green_moon_2d.gm_context import GMContext
from green_moon_2d.gm_scene import GMSceneManager
from green_moon_2d.gm_resources import GMResources


GMGlobalContext: GMContext = GMContext()
GMGlobalScenes: GMSceneManager = GMSceneManager()
GMGlobalConfig: GMConfiguration = GMConfiguration()
GMGlobalResources: GMResources = GMResources()

import logging
logger = logging.getLogger(__name__)


class GMEngine:
    def __init__(self, config_file, first_scene: str):
        logger.debug("Create a new GMEngine.")

        self.first_scene = first_scene
        GMGlobalConfig.load_config(config_file)

    def run(self):
        """
        Starts the engine and runs the game loop.
        """

        logger.info("Run the engine.")

        pygame.init()

        flags = pygame.SHOWN
        if GMGlobalConfig.fullscreen:
            flags = flags | pygame.FULLSCREEN

        GMGlobalContext.screen = pygame.display.set_mode(
            (GMGlobalConfig.screen_width, GMGlobalConfig.screen_height), flags)
        pygame.display.set_caption(GMGlobalConfig.window_title)
        GMGlobalResources.load_resources(GMGlobalConfig.resource_file)
        GMGlobalScenes.start_scene(self.first_scene)

        clock = pygame.time.Clock()

        while not GMGlobalContext.quit_game:
            GMGlobalContext.update_input()
            GMGlobalScenes.update()
            GMGlobalContext.clear_screen()
            GMGlobalScenes.draw()
            pygame.display.flip()
            GMGlobalContext.dt = clock.tick(GMGlobalConfig.fps)

        GMGlobalConfig.save_config()
        pygame.quit()

        logger.info("Exit the engine.")


