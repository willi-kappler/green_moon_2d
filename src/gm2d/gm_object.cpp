/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the object class
*/

// STD includes:
#include <algorithm>

// Local includes:
#include "gm_object.hpp"
#include "gm_exceptions.hpp"


namespace gm2d{
GMObject::GMObject(std::string_view name, bool active = true, int16_t update_order = 0):
    obj_name(name),
    obj_active(active),
    obj_update_order(update_order),
    obj_groups()
{}

[[nodiscard]] GMHandleResult GMObject::gm_handle_message(const GMMessage &message) {
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

        case GMMessageType::AddGroup:
        {
            gm_add_group(std::any_cast<std::string>(message.msg_data));
        }
        break;

        case GMMessageType::RemoveGroup:
        {
            gm_remove_group(std::any_cast<std::string>(message.msg_data));
        }
        break;

        default:
            throw GMUnknownMessageType(message.msg_type, obj_name);
        break;
    }

    return GMHandleResult();
}

void GMObject::gm_update() {
}

void GMObject::gm_add_group(const std::string &grp) {
    for (const std::string& str: obj_groups) {
        if (str == grp) {
            return;
        }
    }

    obj_groups.push_back(grp);
}

void GMObject::gm_remove_group(std::string_view grp) {
    for (size_t i = 0; i < obj_groups.size(); i++) {
        if (obj_groups[i] == grp) {
            std::swap(obj_groups[i], obj_groups.back());
            obj_groups.pop_back();
            return;
        }
    }
}

[[nodiscard]] bool GMObject::gm_is_in_group(std::string_view grp) {
    for (const std::string& str: obj_groups) {
        if (str == grp) {
            return true;
        }
    }

    return false;
}

GMGFXObject::GMGFXObject(std::string_view name, bool visible = true, int16_t draw_order = 0):
    GMObject(name),
    gfx_visible(visible),
    gfx_draw_order(draw_order),
    gfx_pos()
{}

[[nodiscard]] GMHandleResult GMGFXObject::gm_handle_message(const GMMessage &message) {
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

        case GMMessageType::SetPosition:
            gfx_pos = std::any_cast<GMVec2D>(message.msg_data);
        break;

        case GMMessageType::GetPosition:
            return GMHandleResult(GMHandleResultType::Position, gfx_pos);
        break;

        case GMMessageType::AddPosition:
            gfx_pos += std::any_cast<GMVec2D>(message.msg_data);
        break;

        default:
            return GMObject::gm_handle_message(message);
        break;
    }

    return GMHandleResult();
}

void GMGFXObject::gm_draw() {
}

GMObjectManager::GMObjectManager():
    normal_objects(),
    gfx_objects()
{}

void GMObjectManager::gm_update() {
    std::stable_sort(normal_objects.begin(), normal_objects.end(), [](
        const std::unique_ptr<GMObject> &obj1,
        const std::unique_ptr<GMObject> &obj2) {
        return obj1->obj_update_order < obj2->obj_update_order;
    });

    std::stable_sort(gfx_objects.begin(), gfx_objects.end(), [](
        const std::unique_ptr<GMGFXObject> &obj1,
        const std::unique_ptr<GMGFXObject> &obj2) {
        return obj1->obj_update_order < obj2->obj_update_order;
    });

    for (auto &obj: normal_objects) {
        obj->gm_update();
    }

    for (auto &obj: gfx_objects) {
        obj->gm_update();
    }
}

void GMObjectManager::gm_draw() {
    std::stable_sort(gfx_objects.begin(), gfx_objects.end(), [](
        const std::unique_ptr<GMGFXObject> &obj1,
        const std::unique_ptr<GMGFXObject> &obj2) {
        return obj1->gfx_draw_order < obj2->gfx_draw_order;
    });

    for (auto &obj: gfx_objects) {
        obj->gm_draw();
    }
}

void GMObjectManager::gm_add_object(GMObject new_obj) {
    for (auto &o: normal_objects) {
        if (o->obj_name == new_obj.obj_name) {
            throw GMObjectNameDuplicate("GMObjectManager::gm_add_object", o->obj_name);
        }
    }

    normal_objects.push_back(std::make_unique<GMObject>(new_obj));
}

void GMObjectManager::gm_remove_object(std::string_view name) {
    for (size_t i = 0; i < normal_objects.size(); i++) {
        if (normal_objects[i]->obj_name == name) {
            return;
        }
    }

    throw GMObjectNotFound("GMObjectManager::gm_remove_object", name);
}

void GMObjectManager::gm_replace_object(GMObject new_obj) {
    for (size_t i = 0; i < normal_objects.size(); i++) {
        if (normal_objects[i]->obj_name == new_obj.obj_name) {
            normal_objects[i] = std::make_unique<GMObject>(new_obj);
            return;
        }
    }

    throw GMObjectNotFound("GMObjectManager::gm_replace_object", new_obj.obj_name);
}

void GMObjectManager::gm_clear_objects() {
    normal_objects.clear();
}

[[nodiscard]] GMHandleResult GMObjectManager::gm_send_message_object(std::string_view name, const GMMessage &msg) {
    for (auto &o: normal_objects) {
        if (o->obj_name == name) {
            return o->gm_handle_message(msg);
        }
    }

    throw GMObjectNotFound("GMObjectManager::gm_send_message_object", name);
}

void GMObjectManager::gm_add_gfx_object(GMGFXObject new_obj) {
    for (auto &o: gfx_objects) {
        if (o->obj_name == new_obj.obj_name) {
            throw GMObjectNameDuplicate("GMObjectManager::gm_add_gfx_object", o->obj_name);
        }
    }

    gfx_objects.push_back(std::make_unique<GMGFXObject>(new_obj));
}

void GMObjectManager::gm_remove_gfx_object(std::string_view name) {
    for (size_t i = 0; i < gfx_objects.size(); i++) {
        if (gfx_objects[i]->obj_name == name) {
            return;
        }
    }

    throw GMObjectNotFound("GMObjectManager::gm_remove_gfx_object", name);
}

void GMObjectManager::gm_replace_gfx_object(GMGFXObject new_obj) {
    for (size_t i = 0; i < gfx_objects.size(); i++) {
        if (gfx_objects[i]->obj_name == new_obj.obj_name) {
            gfx_objects[i] = std::make_unique<GMGFXObject>(new_obj);
            return;
        }
    }

    throw GMObjectNotFound("GMObjectManager::gm_replace_gfx_object", new_obj.obj_name);
}

void GMObjectManager::gm_clear_gfx_objects() {
    gfx_objects.clear();
}

[[nodiscard]] GMHandleResult GMObjectManager::gm_send_message_gfx_object(std::string_view name, const GMMessage &msg) {
    for (auto &o: gfx_objects) {
        if (o->obj_name == name) {
            return o->gm_handle_message(msg);
        }
    }

    throw GMObjectNotFound("GMObjectManager::gm_send_message_gfx_object", name);
}

}
