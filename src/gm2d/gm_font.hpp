/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the 2D font
*/

#ifndef FILE_GM_FONT_HPP_INCLUDED
#define FILE_GM_FONT_HPP_INCLUDED

// STD includes:
#include <string>
#include <stdfloat>
#include <cstdint>
#include <unordered_map>

// Local includes:
#include "gm_texture.hpp"

namespace gm2d {
class GMFont {
    public:
        GMFont(std::string, GMTexture, std::unordered_map<char, uint16_t>);

        void gm_draw(std::float32_t, std::float32_t, char);

    private:
        std::string name;
        GMTexture texture;
        std::unordered_map<char, uint16_t> mapping;
};
}

#endif // FILE_GM_FONT_HPP_INCLUDED
