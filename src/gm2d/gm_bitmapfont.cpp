/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the bitmap font
*/

// Local includes:
#include "gm_bitmapfont.hpp"

namespace gm2d {
GMBitmapFont::GMBitmapFont(std::string font_name, GMTexture font_texture, std::flat_map<char, uint16_t> font_mapping):
    name(font_name),
    texture(font_texture),
    mapping(font_mapping)
{}

void GMBitmapFont::gm_draw(GMContext &context, const std::float32_t x, const std::float32_t y, const char c) {
    if (auto it = mapping.find(c); it != mapping.end()) {
        texture.gm_draw(context, x, y, it->second);
    }
}
}
