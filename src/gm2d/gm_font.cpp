/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the 2D font
*/

// Local includes:
#include "gm_font.hpp"

namespace gm2d {
GMFont::GMFont(std::string font_name, GMTexture font_texture, std::unordered_map<char, uint16_t> font_mapping):
    name(font_name),
    texture(font_texture),
    mapping(font_mapping)
{}

void GMFont::gm_draw(std::float32_t x, std::float32_t y, char c) {
    texture.gm_draw(x, y, mapping[c]);
}
}
