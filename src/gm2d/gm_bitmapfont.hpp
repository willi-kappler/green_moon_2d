/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the bitmap font
*/

#ifndef FILE_GM_BITMAPFONT_HPP_INCLUDED
#define FILE_GM_BITMAPFONT_HPP_INCLUDED

// STD includes:
#include <string>
#include <stdfloat>
#include <cstdint>
#include <memory>
#include <flat_map>

// Local includes:
#include "gm_texture.hpp"
#include "gm_context.hpp"
#include "gm_math.hpp"

namespace gm2d {
class GMBitmapFont {
    public:
        GMBitmapFont(std::string, std::shared_ptr<GMTexture>, std::flat_map<char, uint16_t>);

        void gm_draw(GMContext &, const std::float32_t, const std::float32_t, const char);
        void gm_draw(GMContext &, const GMVec2D, const char);
        [[nodiscard]] uint16_t gm_char_width();
        [[nodiscard]] uint16_t gm_char_height();

    private:
        const std::string name;
        std::shared_ptr<GMTexture> texture;
        const std::flat_map<char, uint16_t> mapping;
};
}

#endif // FILE_GM_BITMAPFONT_HPP_INCLUDED
