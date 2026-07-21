/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the 2D texture
*/

// Local includes:
#include "gm_texture.hpp"

namespace gm2d {
GMTexture::GMTexture(std::string tex_name, std::shared_ptr<SDL_Texture> tex, uint16_t w, uint16_t h):
    name(tex_name),
    texture(tex),
    src_rect(0, 0, w, h),
    dst_rect(0, 0, w, h),
    flip_mode(),
    unit_width(w),
    unit_height(h),
    columns(tex->w / unit_width)
{
    SDL_SetTextureScaleMode(texture.get(), SDL_SCALEMODE_LINEAR);
}

void GMTexture::gm_set_src_rect(uint16_t index) {
    uint16_t yi = index / columns;
    uint16_t xi = index - (yi * columns);
    src_rect.x = xi * columns;
    src_rect.y = yi * columns;
}

void GMTexture::gm_set_dst_rect(std::float32_t x, std::float32_t y) {
    std::float32_t uw = static_cast<std::float32_t>(unit_width) / 2.0f32;
    std::float32_t uh = static_cast<std::float32_t>(unit_height) / 2.0f32;
    dst_rect.x = x - uw;
    dst_rect.y = y - uh;
}

void GMTexture::gm_draw(GMContext &context, const std::float32_t x, const std::float32_t y, const uint16_t index) {
    gm_set_src_rect(index);
    gm_set_dst_rect(x, y);
    SDL_RenderTexture(context.gm_renderer, texture.get(), &src_rect, &dst_rect);
}

void GMTexture::gm_draw_opt(GMContext &context, const std::float32_t x, const std::float32_t y, const uint16_t index, const std::float32_t angle) {
    gm_set_src_rect(index);
    gm_set_dst_rect(x, y);

    SDL_RenderTextureRotated(context.gm_renderer, texture.get(), &src_rect, &dst_rect, angle, NULL, flip_mode);
}

void GMTexture::gm_set_scale(std::float32_t sx, std::float32_t sy) {
    dst_rect.w = static_cast<std::float32_t>(unit_width) * sx;
    dst_rect.h = static_cast<std::float32_t>(unit_height) * sy;
}

void GMTexture::gm_flip_x(bool flip_x) {
    if (flip_x) {
        flip_mode = static_cast<SDL_FlipMode>(flip_mode | SDL_FLIP_HORIZONTAL);
    } else {
        flip_mode = static_cast<SDL_FlipMode>(flip_mode & ~SDL_FLIP_HORIZONTAL);
    }
}

void GMTexture::gm_flip_y(bool flip_y) {
    if (flip_y) {
        flip_mode = static_cast<SDL_FlipMode>(flip_mode | SDL_FLIP_VERTICAL);
    } else {
        flip_mode = static_cast<SDL_FlipMode>(flip_mode & ~SDL_FLIP_VERTICAL);
    }
}

void GMTexture::gm_flip_xy(bool flip_x, bool flip_y) {
    gm_flip_x(flip_x);
    gm_flip_y(flip_y);
}
}
