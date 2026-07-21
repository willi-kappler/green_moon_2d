/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the context that is passed to the update() and draw() methods.
*/


// Local includes:
#include "gm_context.hpp"

namespace gm2d {
GMContext::GMContext(SDL_Window *sdl_win, SDL_Renderer *sdl_render):
    object_messages(),
    group_messages(),
    objmgr_messages(),
    scene_messages(),
    scenemgr_messages(),
    engine_messages(),
    gm_quit(false),
    gm_dt(0.0f32),
    gm_window(sdl_win),
    gm_is_fullscreen(true),
    gm_renderer(sdl_render),
    gm_game_properties()
{}

void GMContext::gm_send_object_message(const GMObjectMessage &message) {
    object_messages.push_back(message);
}

void GMContext::gm_send_group_message(const GMObjectMessage &message) {
    group_messages.push_back(message);
}

void GMContext::gm_send_objmgr_message(const GMObjMgrMessage &message) {
    objmgr_messages.push_back(message);
}

void GMContext::gm_send_scene_message(const GMSceneMessage &message) {
    scene_messages.push_back(message);
}

void GMContext::gm_send_scenemgr_message(const GMSceneMgrMessage &message) {
    scenemgr_messages.push_back(message);
}

void GMContext::gm_send_engine_message(const GMEngineMessage &message) {
    engine_messages.push_back(message);
}

void GMContext::gm_clear_screen() {
    SDL_RenderClear(gm_renderer);
}

void GMContext::gm_clear_screen(uint8_t r, uint8_t g, uint8_t b) {
    SDL_SetRenderDrawColor(gm_renderer, r, g, b, 0);
    SDL_RenderClear(gm_renderer);
}

void GMContext::gm_set_background_color(uint8_t r, uint8_t g, uint8_t b) {
    SDL_SetRenderDrawColor(gm_renderer, r, g, b, 0);
}

void GMContext::gm_toggle_fullscreen() {
    gm_is_fullscreen = !gm_is_fullscreen;
    SDL_SetWindowFullscreen(gm_window, gm_is_fullscreen);
}

void GMContext::gm_set_fullscreen(bool fullscreen) {
    gm_is_fullscreen = fullscreen;
    SDL_SetWindowFullscreen(gm_window, fullscreen);
}
}
