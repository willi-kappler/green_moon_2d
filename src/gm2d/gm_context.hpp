/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the context that is passed to the update() and draw() methods.
*/

#ifndef FILE_GM_CONTEXT_HPP_INCLUDED
#define FILE_GM_CONTEXT_HPP_INCLUDED

// STD includes:
#include <vector>
#include <stdfloat>
#include <unordered_map>
#include <string>
#include <any>
#include <cstdint>

// External includes:
#include <SDL3/SDL.h>

// Local includes:
#include "gm_message.hpp"

/*
// STD includes:

#include <memory>

*/

namespace gm2d {
class GMContext {
    public:
        GMContext(SDL_Window *, SDL_Renderer *);

        void gm_send_object_message(const GMObjectMessage &);
        void gm_send_group_message(const GMObjectMessage &);
        void gm_send_objmgr_message(const GMObjMgrMessage &);
        void gm_send_scene_message(const GMSceneMessage &);
        void gm_send_scenemgr_message(const GMSceneMgrMessage &);
        void gm_send_engine_message(const GMEngineMessage &);

        std::vector<GMObjectMessage> object_messages;
        std::vector<GMObjectMessage> group_messages;
        std::vector<GMObjMgrMessage> objmgr_messages;
        std::vector<GMSceneMessage> scene_messages;
        std::vector<GMSceneMgrMessage> scenemgr_messages;
        std::vector<GMEngineMessage> engine_messages;

        void gm_clear_screen();
        void gm_clear_screen(uint8_t, uint8_t, uint8_t);
        void gm_set_background_color(uint8_t, uint8_t, uint8_t);
        void gm_toggle_fullscreen();
        void gm_set_fullscreen(bool);

        bool gm_quit;
        std::float32_t gm_dt;
        SDL_Window *gm_window;
        bool gm_is_fullscreen;
        SDL_Renderer *gm_renderer;
        std::unordered_map<std::string, std::any> gm_game_properties;
};
}

#endif // FILE_GM_CONTEXT_HPP_INCLUDED
