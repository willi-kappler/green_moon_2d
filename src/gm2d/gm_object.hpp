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
#include "gm_string_id.hpp"

namespace gm2d {
class GMObject {
    public:
        // Constructor:
        GMObject(GMStringId);
        virtual ~GMObject() = default;

        // Fluent builder pattern:
        [[nodiscard]] GMObject& with_active(bool) &;
        [[nodiscard]] GMObject& with_visible(bool) &;
        [[nodiscard]] GMObject& with_update_order(int16_t) &;
        [[nodiscard]] GMObject& with_draw_order(int16_t) &;
        [[nodiscard]] GMObject& with_position(GMVec2D) &;

        // Handle message:
        virtual void gm_handle_message(const GMMessage &);

        // Send message:
        void gm_send_message(GMMessage);

        // Update:
        void virtual gm_update();

        // Draw:
        void virtual gm_draw();

        // Group:
        void gm_add_group(const GMStringId);
        void gm_remove_group(const GMStringId);
        void gm_clear_groups();
        [[nodiscard]] bool gm_is_in_group(const GMStringId);

        // Member variables:
        const GMStringId obj_name_id;
        bool obj_active;
        bool obj_visible;
        int16_t obj_update_order;
        int16_t obj_draw_order;
        GMVec2D obj_position;
        std::vector<GMMessage> obj_messages;

    private:
        std::vector<GMStringId> obj_groups;
};

class GMObjectManager {
    public:
        GMObjectManager();

        void gm_update();
        void gm_draw();
        void gm_add_object(GMObject);
        void gm_remove_object(GMStringId);
        void gm_replace_object(GMStringId, GMObject);
        void gm_clear_objects();
        void gm_send_message(const GMMessage &);
        void gm_handle_message(const GMMessage &);
        void gm_apply(GMStringId, std::function<void(GMObject &)>);
        void gm_apply_n(std::span<GMStringId>, std::function<void(GMObject &)>);
        void gm_apply_group(GMStringId, std::function<void(GMObject &)>);

        std::vector<GMMessage> scene_messages;

    private:
        std::vector<std::unique_ptr<GMObject>> objects;

};
}

#endif // FILE_GM_OBJECT_HPP_INCLUDED
