# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMContext class which is passed to all the update() and draw() methods.
"""

class GMContext:
    """
    This class contains acess to the scene manager and the state of the game engine.
    It also allows you to store data that is used across multiple scenes.
    """
    def __init__(self):
        game_property = {}




