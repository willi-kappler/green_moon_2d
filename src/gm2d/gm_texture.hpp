/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the 2D texture
*/

#ifndef FILE_GM_TEXTURE_HPP_INCLUDED
#define FILE_GM_TEXTURE_HPP_INCLUDED

// STD includes:
#include <cstdint>
#include <stdfloat>
#include <memory>
#include <string>

// External includes:
#include <SDL3/SDL.h>

// Internal includes:
#include "gm_context.hpp"
#include "gm_math.hpp"

namespace gm2d {
class GMTexture {
    public:
        GMTexture(std::string, std::shared_ptr<SDL_Texture>, uint16_t, uint16_t);

        void gm_draw(GMContext &, const std::float32_t, const std::float32_t, const uint16_t);
        void gm_draw(GMContext &, const GMVec2D, const uint16_t);
        void gm_draw_opt(GMContext &, const std::float32_t, const std::float32_t, const uint16_t, const std::float32_t);
        void gm_draw_opt(GMContext &, const GMVec2D, const uint16_t, const std::float32_t);
        void gm_set_scale(std::float32_t, std::float32_t);
        void gm_flip_x(bool);
        void gm_flip_y(bool);
        void gm_flip_xy(bool, bool);

        const uint16_t gm_unit_width;
        const uint16_t gm_unit_height;
        const uint16_t gm_columns;

    private:
        void gm_set_src_rect(uint16_t);
        void gm_set_dst_rect(std::float32_t, std::float32_t);

        const std::string name;
        const std::shared_ptr<SDL_Texture> texture;
        SDL_FRect src_rect;
        SDL_FRect dst_rect;
        SDL_FlipMode flip_mode;

};
}

#endif // FILE_GM_TEXTURE_HPP_INCLUDED
