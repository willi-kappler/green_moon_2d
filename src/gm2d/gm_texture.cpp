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
    gm_unit_width(w),
    gm_unit_height(h),
    gm_columns(static_cast<uint16_t>(tex->w) / gm_unit_width),
    name(tex_name),
    texture(tex),
    src_rect(0, 0, w, h),
    dst_rect(0, 0, w, h),
    flip_mode(),
    gm_unit_width2(static_cast<float>(gm_unit_width) / 2.0f),
    gm_unit_height2(static_cast<float>(gm_unit_height) / 2.0f)
{
    SDL_SetTextureScaleMode(texture.get(), SDL_SCALEMODE_LINEAR);
}

void GMTexture::gm_set_src_rect(uint16_t index) {
    uint16_t yi = index / gm_columns;
    uint16_t xi = static_cast<uint16_t>(index - (yi * gm_columns));
    src_rect.x = xi * gm_columns;
    src_rect.y = yi * gm_columns;
}

void GMTexture::gm_set_dst_rect(float x, float y) {
    dst_rect.x = x - gm_unit_width2;
    dst_rect.y = y - gm_unit_height2;
}

void GMTexture::gm_draw(GMContext &context, const float x, const float y, const uint16_t index) {
    gm_set_src_rect(index);
    gm_set_dst_rect(x, y);

    context.gm_draw_tex(texture, src_rect, dst_rect);
}

void GMTexture::gm_draw(GMContext &context, const GMVec2D pos, const uint16_t index) {
    gm_draw(context, pos.x, pos.y, index);
}

void GMTexture::gm_draw_opt(GMContext &context, const float x, const float y, const uint16_t index, const float angle) {
    gm_set_src_rect(index);
    gm_set_dst_rect(x, y);

    context.gm_draw_tex_opt(texture, src_rect, dst_rect, angle, flip_mode);
}

void GMTexture::gm_draw_opt(GMContext &context, const GMVec2D pos, const uint16_t index, const float angle) {
    gm_draw_opt(context, pos.x, pos.y, index, angle);
}


void GMTexture::gm_set_scale(float sx, float sy) {
    dst_rect.w = static_cast<float>(gm_unit_width) * sx;
    dst_rect.h = static_cast<float>(gm_unit_height) * sy;
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
