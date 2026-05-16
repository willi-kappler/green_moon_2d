/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the object class
*/

// Local includes:
#include "gm_object.hpp"
#include "gm_exceptions.hpp"

GMObject::GMObject(std::string_view name, bool active = true, int16_t update_order = 0):
    obj_name(name),
    obj_active(active),
    obj_update_order(update_order)
{}

[[nodiscard]] GMHandleResult GMObject::handle_message(const GMMessage &message) {
    switch (message.msg_type) {
        case GMMessageType::SetActive:
            obj_active = std::any_cast<bool>(message.msg_data);
        break;

        case GMMessageType::GetActive:
            return GMHandleResult(GMHandleResultType::Active, obj_active);
        break;

        case GMMessageType::ToggleActive:
            obj_active = !obj_active;
        break;

        case GMMessageType::SetUpdateOrder:
            obj_update_order = std::any_cast<int16_t>(message.msg_data);
        break;

        case GMMessageType::GetUpdateOrder:
            return GMHandleResult(GMHandleResultType::UpdateOrder, obj_update_order);
        break;

        default:
            throw GMUnknownMessageType(message.msg_type, obj_name);
        break;
    }

    return GMHandleResult();
}

GMGFXObject::GMGFXObject(std::string_view name, bool visible = true, int16_t draw_order = 0):
    GMObject(name),
    gfx_visible(visible),
    gfx_draw_order(draw_order)
{}

[[nodiscard]] GMHandleResult GMGFXObject::handle_message(const GMMessage &message) {
    switch(message.msg_type) {
        case GMMessageType::SetVisible:
            gfx_visible = std::any_cast<bool>(message.msg_data);
        break;

        case GMMessageType::GetVisible:
            return GMHandleResult(GMHandleResultType::Visible, gfx_visible);
        break;

        case GMMessageType::ToggleVisible:
            gfx_visible = !gfx_visible;
        break;

        case GMMessageType::SetDrawOrder:
            gfx_draw_order = std::any_cast<int16_t>(message.msg_data);
        break;

        case GMMessageType::GetDrawOrder:
            return GMHandleResult(GMHandleResultType::DrawOrder, gfx_draw_order);
        break;

        default:
            return GMObject::handle_message(message);
        break;
    }

    return GMHandleResult();
}
