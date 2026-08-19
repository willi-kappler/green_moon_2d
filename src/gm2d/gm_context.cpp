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
GMContext::GMContext():
    object_messages(),
    group_messages(),
    objmgr_messages(),
    scene_messages(),
    scenemgr_messages(),
    engine_messages(),
    gm_quit(false),
    gm_dt(0.0f32),
    gm_window(nullptr),
    gm_is_fullscreen(true),
    gm_renderer(nullptr),
    gm_game_properties()
{}

GMContext::~GMContext() {
    gm_destroy_renderer();
    gm_destroy_window();
}

void GMContext::gm_set_window(SDL_Window *sdl_window) {
    gm_window = sdl_window;
}

SDL_Window *GMContext::gm_get_window() const {
    return gm_window;
}

void GMContext::gm_destroy_window() {
    if (gm_window) {
        SDL_DestroyWindow(gm_window);
        gm_window = nullptr;
    }
}

void GMContext::gm_set_renderer(SDL_Renderer *sdl_renderer) {
    gm_renderer = sdl_renderer;
}

SDL_Renderer *GMContext::gm_get_renderer() const {
    return gm_renderer;
}

void GMContext::gm_destroy_renderer() {
    if (gm_renderer) {
        SDL_DestroyRenderer(gm_renderer);
        gm_renderer = nullptr;
    }
}

void GMContext::gm_quit_game() {
    gm_quit = true;
}

bool GMContext::gm_game_running() const {
    return gm_quit;
}

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

std::vector<GMObjectMessage> GMContext::gm_get_object_messages() {
    return std::move(object_messages);
}

std::vector<GMObjectMessage> GMContext::gm_get_group_messages() {
    return std::move(group_messages);
}

std::vector<GMObjMgrMessage> GMContext::gm_get_objmgr_messages() {
    return std::move(objmgr_messages);
}

std::vector<GMSceneMessage> GMContext::gm_get_scene_messages() {
    return std::move(scene_messages);
}

std::vector<GMSceneMgrMessage> GMContext::gm_get_scenemgr_messages() {
    return std::move(scenemgr_messages);
}

std::vector<GMEngineMessage> GMContext::gm_get_engine_messages() {
    return std::move(engine_messages);
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
