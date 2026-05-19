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
        [[nodiscard]] virtual GMHandleResult gm_handle_message(const GMMessage &);

        // Update:
        void virtual gm_update();

        // Group
        void gm_add_group(const std::string &);
        void gm_remove_group(std::string_view);
        [[nodiscard]] bool gm_is_in_group(std::string_view);

        const std::string obj_name;
        bool obj_active;
        int16_t obj_update_order;

    private:
        std::vector<std::string> obj_groups;
};

class GMGFXObject: public GMObject {
    public:
        // Constructor:
        GMGFXObject(std::string_view, bool, int16_t);
        virtual ~GMGFXObject() = default;

        // Handle message:
        [[nodiscard]] GMHandleResult gm_handle_message(const GMMessage &) override;

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

        void gm_add_object(GMObject);
        void gm_remove_object(std::string_view);
        void gm_replace_object(GMObject);
        void gm_clear_objects();
        [[nodiscard]] GMHandleResult gm_send_message_object(std::string_view, const GMMessage &);
        // TODO: send message to group
        // TODO: find object

        void gm_add_gfx_object(GMGFXObject);
        void gm_remove_gfx_object(std::string_view);
        void gm_replace_gfx_object(GMGFXObject);
        void gm_clear_gfx_objects();
        [[nodiscard]] GMHandleResult gm_send_message_gfx_object(std::string_view, const GMMessage &);
        // TODO: send message to gfx group
        // TODO: find gfx object

    private:
        std::vector<std::unique_ptr<GMObject>> normal_objects;
        std::vector<std::unique_ptr<GMGFXObject>> gfx_objects;

};
}

#endif // FILE_GM_OBJECT_HPP_INCLUDED
