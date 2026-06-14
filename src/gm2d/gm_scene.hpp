/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the scene manager
*/

#ifndef FILE_GM_SCENE_HPP_INCLUDED
#define FILE_GM_SCENE_HPP_INCLUDED

// STD includes:
#include <cstdint>
#include <vector>
#include <memory>

// Local includes:
#include "gm_string_id.hpp"
#include "gm_message.hpp"
#include "gm_context.hpp"

namespace gm2d {
class GMScene {
    public:
        GMScene(GMStringId);
        virtual ~GMScene() = default;

        virtual void gm_handle_message(const GMSceneMessage &, GMContext &) = 0;
        virtual void gm_update(GMContext &) = 0;
        virtual void gm_draw(GMContext &) = 0;
        virtual void gm_enter(GMStringId) = 0;

        GMStringId name_id;
        bool on_stack;

    // TODO: Maybe add a custom property ?
};

class GMSceneManager {
    public:
        GMSceneManager();

        void gm_update(GMContext &);
        void gm_draw(GMContext &);
        void gm_add_scene(std::shared_ptr<GMScene>);
        void gm_remove_scene(GMStringId);
        void gm_replace_scene(std::shared_ptr<GMScene>);
        void gm_change_to_scene(GMStringId);
        void gm_push_and_change(GMStringId);
        void gm_pop_and_change();
        void gm_set_start_scene(GMStringId);
        void gm_update_stack_top(GMContext &);
        void gm_draw_stack_top(GMContext &);
        void gm_update_scene(GMStringId, GMContext &);
        void gm_draw_scene(GMStringId, GMContext &);
        void gm_handle_message(const GMSceneMgrMessage &, GMContext &);

    private:
        std::vector<std::shared_ptr<GMScene>> scenes;
        std::shared_ptr<GMScene> current_scene;
        std::vector<std::shared_ptr<GMScene>> scene_stack;
};
}

#endif // FILE_GM_SCENE_HPP_INCLUDED
