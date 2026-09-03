/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the main library file
*/

// Local includes:
#include "gm2d.hpp"
#include "gm_exceptions.hpp"
#include "gm_resources.hpp"

namespace gm2d {
GM2D::GM2D():
    GM2D("config.json")
{}

GM2D::GM2D(std::filesystem::path config_file):
    GM2D(gm_config_from_file(config_file))
{}

GM2D::GM2D(GMConfiguration config):
    configuration(config),
    scene_manager(),
    context()
{
    if (!SDL_Init(SDL_INIT_VIDEO)) {
        SDL_Log("SDL_Init Error: %s", SDL_GetError());
        SDL_Quit();
        throw GMSDLInitFailed(SDL_GetError());
    }

    // TODO: move create window into context class

    SDL_Window *sdl_window = SDL_CreateWindow(config.window_title.c_str(), config.screen_width, config.screen_height, 0);
    if (!sdl_window) {
        SDL_Log("Window creation failed: %s", SDL_GetError());
        SDL_Quit();
        throw GMSDLWindowFailed(SDL_GetError());
    }

    context.gm_set_window(sdl_window);

    // TODO: move create renderer into context class

    SDL_Renderer *sdl_renderer = SDL_CreateRenderer(context.gm_get_window(), NULL);
    if (!sdl_renderer) {
        SDL_Log("Renderer creation failed: %s", SDL_GetError());
        context.gm_destroy_window();
        SDL_Quit();
        throw GMSDLRendererFailed(SDL_GetError());
    }

    context.gm_set_renderer(sdl_renderer);
}

GM2D::~GM2D() {
    gm_clean_up();
}

void GM2D::gm_add_scene(std::shared_ptr<GMScene> new_scene) {
    scene_manager.gm_add_scene(new_scene);
}

void GM2D::gm_set_start_scene(GMStringId start_id) {
    scene_manager.gm_set_start_scene(start_id);
}

void GM2D::gm_run() {
    SDL_Event event;
    GMResourceManager resource_manager;

    while (!context.gm_game_running()) {
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_EVENT_QUIT) {
                context.gm_quit_game();
            }
        }

        scene_manager.gm_update(context);
        scene_manager.gm_draw(context);
    }

    gm_clean_up();
}

void GM2D::gm_clean_up() {
    SDL_Log("Game quit");

    context.gm_destroy_renderer();
    context.gm_destroy_window();

    SDL_Quit();
}
}
