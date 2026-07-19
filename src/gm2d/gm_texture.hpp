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

namespace gm2d {
class GMTexture {
    public:
        GMTexture(std::string, std::shared_ptr<SDL_Texture>, SDL_Renderer *, uint16_t, uint16_t);

        void gm_draw(std::float32_t, std::float32_t, uint16_t);
        void gm_draw_opt(std::float32_t, std::float32_t, uint16_t, std::float32_t);
        void gm_set_scale(std::float32_t, std::float32_t);
        void gm_flip_x(bool);
        void gm_flip_y(bool);
        void gm_flip_xy(bool, bool);


    private:
        void gm_set_src_rect(uint16_t);
        void gm_set_dst_rect(std::float32_t, std::float32_t);

        std::string name;
        std::shared_ptr<SDL_Texture> texture;
        SDL_Renderer *renderer;
        SDL_FRect src_rect;
        SDL_FRect dst_rect;
        SDL_FlipMode flip_mode;

        uint16_t unit_width;
        uint16_t unit_height;
        uint16_t columns;
};

}

#endif // FILE_GM_TEXTURE_HPP_INCLUDED
