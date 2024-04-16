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
import gm_object
import gm_resources
import gm_scene

# Global variables:
var GMGlobConfig*: GMConfiguration
var GMGlobResources*: GMResourceManager
var GMGlobScenes*: GMSceneManager
var GMGlobObjects*: GMObjectManager
var GMGlobGFXObjects*: GMGFXObjectManager

proc gm_init*(configFilename: string = "config.json") =
    GMGlobConfig = gmLoadConfiguration(configFilename)
    GMGlobResources = gmLoadResources(GMGlobConfig.gmGetResourcesFilename())
    GMGlobScenes = gmInitSceneManager()
    GMGlobObjects = gmInitObjectManager()
    GMGlobGFXObjects = gmInitGFXObjectManager()


