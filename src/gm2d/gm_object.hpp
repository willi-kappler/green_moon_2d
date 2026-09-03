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
#include <cstdint>
#include <vector>
#include <memory>
#include <utility>
#include <span>
#include <functional>
#include <any>

// Local includes:
#include "gm_message.hpp"
#include "gm_math.hpp"
#include "gm_string_id.hpp"
#include "gm_context.hpp"

namespace gm2d {
class GMObject {
    public:
        // Constructor:
        GMObject(GMStringId);
        virtual ~GMObject() = default;

        // Fluent builder pattern:
        [[nodiscard]] GMObject& with_active(bool);
        [[nodiscard]] GMObject& with_visible(bool);
        [[nodiscard]] GMObject& with_update_order(int16_t);
        [[nodiscard]] GMObject& with_draw_order(int16_t);
        [[nodiscard]] GMObject& with_position(GMVec2D);

        // Message:
        virtual void gm_handle_message(const GMObjectMessage &, GMContext &);

        // Engine:
        virtual void gm_update(GMContext &) = 0;
        virtual void gm_draw(GMContext &) = 0;

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
        GMVec2D obj_pos_delta;

    private:
        std::vector<GMStringId> obj_groups;
};

class GMObjectManager {
    public:
        GMObjectManager(uint8_t);

        void gm_add_layer();
        void gm_remove_layer(uint8_t);
        void gm_clear_layer(uint8_t);
        void gm_clear_all_layers();
        void gm_update(GMContext &);
        void gm_draw(GMContext &);
        void gm_add_object(std::shared_ptr<GMObject>, uint8_t);
        void gm_remove_object(GMStringId, uint8_t);
        void gm_replace_object(std::shared_ptr<GMObject>, uint8_t);
        void gm_clear_all_objects();
        void gm_move_to_layer1(GMStringId, uint8_t);
        void gm_move_to_layer2(GMStringId, uint8_t, uint8_t);
        void gm_handle_message(GMObjMgrMessage &);
        void gm_apply(GMStringId, std::function<void(GMObject &)>, uint8_t);
        void gm_apply_n(std::span<GMStringId>, std::function<void(GMObject &)>, uint8_t);
        void gm_apply_n(std::span<GMStringId>, std::function<void(GMObject &)>);
        void gm_apply_group(GMStringId, std::function<void(GMObject &)>, uint8_t);
        void gm_apply_group(GMStringId, std::function<void(GMObject &)>);

    private:
        std::vector<std::vector<std::shared_ptr<GMObject>>> layers;

};
}

#endif // FILE_GM_OBJECT_HPP_INCLUDED
