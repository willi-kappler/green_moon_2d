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
#include <unordered_map>
#include <string>
#include <any>
#include <cstdint>

// STD includes:
// #include <string_view>
// #include <memory>


// External includes:
#include <SDL3/SDL.h>

// Local includes:
#include "gm_message.hpp"


namespace gm2d {
class GMContext {
    public:
        GMContext();
        ~GMContext();

        void gm_set_window(SDL_Window *);
        SDL_Window *gm_get_window() const;
        void gm_destroy_window();
        void gm_set_renderer(SDL_Renderer *);
        SDL_Renderer *gm_get_renderer() const;
        void gm_destroy_renderer();

        void gm_quit_game();
        bool gm_game_running() const;

        void gm_send_object_message(const GMObjectMessage &);
        void gm_send_group_message(const GMObjectMessage &);
        void gm_send_objmgr_message(const GMObjMgrMessage &);
        void gm_send_scene_message(const GMSceneMessage &);
        void gm_send_scenemgr_message(const GMSceneMgrMessage &);
        void gm_send_engine_message(const GMEngineMessage &);

        std::vector<GMObjectMessage> gm_get_object_messages();
        std::vector<GMObjectMessage> gm_get_group_messages();
        std::vector<GMObjMgrMessage> gm_get_objmgr_messages();
        std::vector<GMSceneMessage> gm_get_scene_messages();
        std::vector<GMSceneMgrMessage> gm_get_scenemgr_messages();
        std::vector<GMEngineMessage> gm_get_engine_messages();

        void gm_clear_screen();
        void gm_clear_screen(uint8_t, uint8_t, uint8_t);
        void gm_set_background_color(uint8_t, uint8_t, uint8_t);
        void gm_toggle_fullscreen();
        void gm_set_fullscreen(bool);

        float gm_get_dt();

    private:
        std::vector<GMObjectMessage> object_messages;
        std::vector<GMObjectMessage> group_messages;
        std::vector<GMObjMgrMessage> objmgr_messages;
        std::vector<GMSceneMessage> scene_messages;
        std::vector<GMSceneMgrMessage> scenemgr_messages;
        std::vector<GMEngineMessage> engine_messages;

        bool gm_quit;
        float gm_dt;
        SDL_Window *gm_window;
        bool gm_is_fullscreen;
        SDL_Renderer *gm_renderer;
        std::unordered_map<std::string, std::any> gm_game_properties;
};
}

#endif // FILE_GM_CONTEXT_HPP_INCLUDED
