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

namespace gm2d {
GM2D::GM2D():
    GM2D("config.json")
{}

GM2D::GM2D(std::filesystem::path config_file):
    GM2D(gm_config_from_file(config_file))
{}

GM2D::GM2D(GMConfiguration config):
    configuration(config),
    context(NULL, NULL)
{
    if (!SDL_Init(SDL_INIT_VIDEO)) {
        SDL_Log("SDL_Init Error: %s", SDL_GetError());
        throw GMSDLInitFailed(SDL_GetError());
    }

    context.gm_window = SDL_CreateWindow(config.window_title.c_str(), config.screen_width, config.screen_height, 0);
    if (context.gm_window) {
        SDL_Log("Window creation failed: %s", SDL_GetError());
        SDL_Quit();
        throw GMSDLWindowFailed(SDL_GetError());
    }

    context.gm_renderer = SDL_CreateRenderer(context.gm_window, NULL);
    if (!context.gm_renderer) {
        SDL_Log("Renderer creation failed: %s", SDL_GetError());
        SDL_DestroyWindow(context.gm_window);
        SDL_Quit();
        throw GMSDLRendererFailed(SDL_GetError());
    }
}

void GM2D::gm_add_scene(std::shared_ptr<GMScene> new_scene) {
    scn_manager.gm_add_scene(new_scene);
}

void GM2D::gm_set_start_scene(GMStringId start_id) {
    scn_manager.gm_set_start_scene(start_id);
}

void GM2D::gm_run() {
    SDL_Event event;

    while (!context.gm_quit) {
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_EVENT_QUIT) {
                context.gm_quit = true;
            }
        }

        scn_manager.gm_update(context);
        scn_manager.gm_draw(context);
    }


    SDL_Log("Game quit");
    SDL_DestroyRenderer(context.gm_renderer);
    SDL_DestroyWindow(context.gm_window);
    SDL_Quit();
}


}
