# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import json

import sdl2
#import sdl2.ext
import sdl2.sdlimage


import green_moon_2d.gm_animation as gmanim
from green_moon_2d.gm_texture import GMTexture
from green_moon_2d.gm_font import GMFont
from green_moon_2d.gm_context import GMContext

class GMResources:
    def __init__(self):
        self.textures: dict[str, GMTexture] = {}
        self.animations: dict[str, gmanim.GMAnimation] = {}
        self.fonts: dict[str, GMFont] = {}

    def load_resources(self, file_name: str, context: GMContext) -> None:
        """
        Load resources (JSON format) from the given file name.

        :param file_name: File name of the resource file.
        """

        with open(file_name, "r") as f:
            data = json.load(f)

        if "textures" in data:
            self.create_textures(data["texture"], context)

        if "fonts" in data:
            self.create_fonts(data["fonts"])

        if "animations" in data:
            self.create_animations(data["animations"])

    def create_textures(self, textures: list, context: GMContext) -> None:
        """
        Load and create the texutres based on the resource file.

        :param textures: A list of file names.
        """

        for tex in textures:
            name: str = tex["name"]
            file_name: str = tex["file_name"]
            unit_width: int = tex["unit_width"]
            unit_height: int = tex["unit_height"]

            #sdl_texture = sdl2.ext.image.load_img(file_name)
            sdl_texture: sdl2.SDL_Texture = sdl2.sdlimage.IMG_LoadTexture(context.renderer, file_name)
            new_texture = GMTexture(unit_width, unit_height, sdl_texture)
            self.textures[name] = new_texture

    def create_fonts(self, fonts: list) -> None:
        """
        Creates the fonts based on the resource file.

        :param fonts: a list of font definitions.
        """

        for font in fonts:
            name: str = font["name"]
            texture_name: str = font["texture_name"]
            char_mapping: list[str] = font["char_mapping"]
            texture: GMTexture = self.textures[texture_name]
            mapping: dict[str, int] = {}

            for i, c in enumerate(char_mapping):
                mapping[c] = i

            new_font = GMFont(texture, mapping)
            self.fonts[name] = new_font

    def create_animations(self, animations: list) -> None:
        """
        Create the animations based on the resource file.

        :param animations: a list of animation definitions.
        """

        for anim in animations:
            name: str = anim["name"]
            animation_type: str = anim["animtaion_type"]
            frames: list[tuple[int, int]] = anim["frames"]

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

            self.animations[name] = new_anim


