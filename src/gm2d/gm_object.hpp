/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the object class
*/

#ifndef FILE_GM_OBJECT_HPP_INCLUDED
#define FILE_GM_OBJECT_HPP_INCLUDED

// STD includes:
#include <string>
#include <cstdint>
#include <string_view>
#include <vector>
#include <memory>
#include <utility>
#include <span>
#include <functional>

// Local includes:
#include "gm_message.hpp"
#include "gm_math.hpp"


namespace gm2d {

class GMObject {
    public:
        // Constructor:
        GMObject(std::string_view, bool, int16_t);
        virtual ~GMObject() = default;

        // Handle message:
        virtual void gm_handle_message(const GMMessage &);

        // Send message:
        void gm_send_normal_message(GMMessage);
        void gm_send_normal_message_group(GMMessage);
        void gm_send_gfx_message(GMMessage);
        void gm_send_gfx_message_group(GMMessage);
        void gm_send_combined_group(GMMessage);
        void gm_send_obj_mgr(GMMessage);
        void gm_send_scene_mgr(GMMessage);

        // Update:
        void virtual gm_update();

        // Group
        void gm_add_group(const std::string &);
        void gm_remove_group(std::string_view);
        void gm_clear_groups();
        [[nodiscard]] bool gm_is_in_group(std::string_view);

        const std::string obj_name;
        bool obj_active;
        int16_t obj_update_order;
        std::vector<GMMessage> normal_messages;
        std::vector<GMMessage> normal_group_messages;
        std::vector<GMMessage> gfx_messages;
        std::vector<GMMessage> gfx_group_messages;
        std::vector<GMMessage> combined_group_messages;
        std::vector<GMMessage> obj_mgr_messages;
        std::vector<GMMessage> scene_mgr_messages;

    private:
        std::vector<std::string> obj_groups;
};

class GMGFXObject: public GMObject {
    public:
        // Constructor:
        GMGFXObject(std::string_view, bool, int16_t);
        virtual ~GMGFXObject() = default;

        // Handle message:
        void gm_handle_message(const GMMessage &) override;

        // Draw:
        void virtual gm_draw();

        bool gfx_visible;
        int16_t gfx_draw_order;
        GMVec2D gfx_pos;
};

class GMObjectManager {
    public:
        GMObjectManager();

        void gm_update();

        void gm_draw();

        /*
        void gm_add_object(GMObject, GMMessageCategory);
        void gm_remove_object(std::string_view, GMMessageCategory);
        void gm_replace_object(GMObject, GMMessageCategory);
        void gm_clear_objects(GMMessageCategory);
        void gm_send_message(const GMMessage &);
        void gm_apply(std::span<std::string_view>, GMMessageCategory, std::function<void(GMObject &)>);
        */

    private:
        std::vector<std::unique_ptr<GMObject>> normal_objects;
        std::vector<std::unique_ptr<GMGFXObject>> gfx_objects;

};
}

#endif // FILE_GM_OBJECT_HPP_INCLUDED
