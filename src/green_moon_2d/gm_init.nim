## This module is part of GreenMoon2D: https://github.com/willi-kappler/green_moon_2d
##
## Written by Willi Kappler, License: MIT
##
## This Nim library allows you to write 2D games, it used Naylib (Raylib) internally.
##
## This module contains the main data structure and initialisazion routine.
## The game starts from here, this is the main entry point of execution.
##

# Local imports
import gm_configuration
import gm_log
import gm_object
import gm_resources
import gm_scene

proc gmRun*() =
    while not gmQuitGame():
        discard

proc gmInit*(configFilename: string = "config.json") =
    gmLoadConfiguration(configFilename)
    gmInitLogger()
    gmLoadResources()
    gmInitSceneManager()
    gmInitObjectManager()
    gmInitGFXObjectManager()

