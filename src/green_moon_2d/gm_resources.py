# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import json

import green_moon_2d.gm_animation as gmanim


class GMResources:
    def __init__(self):
        self.texture = {}
        self.animations = {}
        self.fonts = {}

    def load_resources(self, file_name: str) -> None:
        """
        Load resources (JSON format) from the given file name.

        :param file_name: File name of the resource file.
        """

        with open(file_name, "r") as f:
            data = json.load(f)

        if "textures" in data:
            self.create_textures(data["texture"])

        if "fonts" in data:
            self.create_fonts(data["fonts"])

        if "animations" in data:
            self.create_animations(data["animations"])

    def create_textures(self, textures: list) -> None:
        """
        Load and create the texutres based on the resource file.

        :param textures: A list of file names.
        """

        for tex in textures:
            name: str = tex["name"]
            file_name: str = tex["file_name"]
            unit_width: int = tex["unit_width"]
            unit_height: int = tex["unit_height"]


    def create_fonts(self, fonts: list) -> None:
        """
        Creates the fonts based on the resource file.

        :param fonts: a list of font definitions.
        """

        for font in fonts:
            name = font["name"]
            texture_name = font["texture_name"]
            char_mapping = font["char_mapping"]

    def create_animations(self, animations: list) -> None:
        """
        Create the animations based on the resource file.

        :param animations: a list of animation definitions.
        """

        for anim in animations:
            name = anim["name"]
            animation_type = anim["animtaion_type"]
            frames = anim["frames"]

            new_anim = gmanim.GMAnimation(frames)

            match animation_type:
                case "forward":
                    new_anim.change_type(gmanim.GMAnimType.FORWARD)
                case "backward":
                    new_anim.change_type(gmanim.GMAnimType.BACKWARD)
                case "loop_forward":
                    new_anim.change_type(gmanim.GMAnimType.FORWARD_LOOP)
                case "loop_backward":
                    new_anim.change_type(gmanim.GMAnimType.BACKWARD_LOOP)
                case "ping_pong_forward":
                    new_anim.change_type(gmanim.GMAnimType.PINGPONG_F)
                case "ping_pong_backward":
                    new_anim.change_type(gmanim.GMAnimType.PINGPONG_B)
                case _:
                    raise ValueError(f"GMResources, Unknown animation type: {animation_type}, name: {name}")


            
