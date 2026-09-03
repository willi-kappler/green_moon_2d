/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the main library file
*/

#ifndef FILE_GM_GM2D_HPP_INCLUDED
#define FILE_GM_GM2D_HPP_INCLUDED

// STD include:
#include <string_view>
#include <filesystem>
#include <memory>

// External include:
#include <SDL3/SDL.h>

// Local include:
#include "gm_configuration.hpp"
#include "gm_string_id.hpp"
#include "gm_scene.hpp"
#include "gm_context.hpp"

namespace gm2d {
class GM2D {
    public:
        GM2D();
        GM2D(std::filesystem::path);
        GM2D(GMConfiguration);
        ~GM2D();

        void gm_add_scene(std::shared_ptr<GMScene>);
        void gm_set_start_scene(GMStringId);
        void gm_run();

    private:
        GMConfiguration configuration;
        GMSceneManager scene_manager;
        GMContext context;

        void gm_clean_up();
    };
}


#endif // FILE_GM_GM2D_HPP_INCLUDED
