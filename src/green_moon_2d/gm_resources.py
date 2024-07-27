# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import json
from typing import override

import pygame

import green_moon_2d.gm_animation as gmanim
import green_moon_2d.gm_texture as gmtxt
import green_moon_2d.gm_font as gmfnt
import green_moon_2d.gm_interfaces as gmitf


class GMResources(gmitf.GMResourcesInterface):
    def __init__(self):
        self.textures: dict[str, gmtxt.GMTexture] = {}
        self.animations: dict[str, gmanim.GMAnimation] = {}
        self.fonts: dict[str, gmfnt.GMFont] = {}

    @override
    def load_resources(self, file_name: str) -> None:
        """
        Load resources (JSON format) from the given file name.

        :param file_name: File name of the resource file.
        """

        with open(file_name, "r") as f:
            data = json.load(f)

        if "textures" in data:
            self.create_textures(data["textures"])

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

            surface = pygame.image.load(file_name)
            new_texture = gmtxt.GMTexture(unit_width, unit_height, surface)
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
            texture: gmtxt.GMTexture = self.textures[texture_name]
            mapping: dict[str, int] = {}

            for i, c in enumerate(char_mapping):
                mapping[c] = i

            new_font = gmfnt.GMFont(texture, mapping)
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

    def get_texture(self, name: str) -> gmtxt.GMTexture:
        return self.textures[name]

    def get_font(self, name: str) -> gmfnt.GMFont:
        return self.fonts[name]

    def get_animation(self, name: str) -> gmanim.GMAnimation:
        return self.animations[name]


