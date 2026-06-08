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
GMObject::GMObject(GMStringId name_id):
    obj_name_id(name_id),
    obj_active(false),
    obj_visible(false),
    obj_update_order(0),
    obj_draw_order(0),
    obj_position(),
    obj_groups()
{}

[[nodiscard]] GMObject& GMObject::with_active(bool active) & {
    obj_active = active;
    return *this;
}

[[nodiscard]] GMObject& GMObject::with_visible(bool visible) & {
    obj_visible = visible;
    return *this;
}

[[nodiscard]] GMObject& GMObject::with_update_order(int16_t update_order) & {
    obj_update_order = update_order;
    return *this;
}

[[nodiscard]] GMObject& GMObject::with_draw_order(int16_t draw_order) & {
    obj_draw_order = draw_order;
    return *this;
}

[[nodiscard]] GMObject& GMObject::with_position(GMVec2D position) & {
    obj_position = position;
    return *this;
}

void GMObject::gm_handle_message(const GMObjectMessage &message) {
    switch (message.msg_type) {
        case GMObjectMessageType::SetActive:
            obj_active = std::any_cast<bool>(message.msg_data);
        break;

        case GMObjectMessageType::GetActive:
            //gm_reply_message(message.msg_sender, GMObjectMessageType::GetActiveResult, obj_active);
        break;

        case GMObjectMessageType::ToggleActive:
            obj_active = !obj_active;
        break;

        case GMObjectMessageType::SetUpdateOrder:
            obj_update_order = std::any_cast<int16_t>(message.msg_data);
        break;

        case GMObjectMessageType::GetUpdateOrder:
            //gm_reply_message(message.msg_sender, GMObjectMessageType::GetUpdateOrderResult, obj_update_order);
        break;

        case GMObjectMessageType::AddGroup:
            gm_add_group(std::any_cast<GMStringId>(message.msg_data));
        break;

        case GMObjectMessageType::RemoveGroup:
            gm_remove_group(std::any_cast<GMStringId>(message.msg_data));
        break;

        case GMObjectMessageType::ClearGroups:
            gm_clear_groups();
        break;

        case GMObjectMessageType::SetVisible:
            obj_visible = std::any_cast<bool>(message.msg_data);
        break;

        case GMObjectMessageType::GetVisible:
            //gm_reply_message(message.msg_sender, GMObjectMessageType::GetVisibleResult, obj_visible);
        break;

        case GMObjectMessageType::ToggleVisible:
            obj_visible = !obj_visible;
        break;

        case GMObjectMessageType::SetDrawOrder:
            obj_draw_order = std::any_cast<int16_t>(message.msg_data);
        break;

        case GMObjectMessageType::GetDrawOrder:
            //gm_reply_message(message.msg_sender, GMObjectMessageType::GetDrawOrderResult, obj_draw_order);
        break;

        case GMObjectMessageType::SetPosition:
            obj_position = std::any_cast<GMVec2D>(message.msg_data);
        break;

        case GMObjectMessageType::GetPosition:
            //gm_reply_message(message.msg_sender, GMObjectMessageType::GetPositionResult, obj_position);
        break;

        case GMObjectMessageType::AddPosition:
            obj_position += std::any_cast<GMVec2D>(message.msg_data);
        break;

        default:
            throw GMUnknownMessageType(message.msg_type, obj_name_id);
        break;
    }
}

void GMObject::gm_update() {
    throw GMMethodNotImplemented("GMObject::gm_update", obj_name_id);
}

void GMObject::gm_draw() {
    throw GMMethodNotImplemented("GMObject::gm_draw", obj_name_id);
}

void GMObject::gm_add_group(const GMStringId grp) {
    for (const GMStringId& str: obj_groups) {
        if (str == grp) {
            return;
        }
    }

    obj_groups.push_back(grp);
}

void GMObject::gm_remove_group(const GMStringId grp) {
    for (size_t i = 0; i < obj_groups.size(); i++) {
        if (obj_groups[i] == grp) {
            std::swap(obj_groups[i], obj_groups.back());
            obj_groups.pop_back();
            return;
        }
    }
}

void GMObject::gm_clear_groups() {
    obj_groups.clear();
}

[[nodiscard]] bool GMObject::gm_is_in_group(const GMStringId grp) {
    for (const GMStringId& str: obj_groups) {
        if (str == grp) {
            return true;
        }
    }

    return false;
}

GMObjectManager::GMObjectManager():
    objects()
{}

void GMObjectManager::gm_update() {
    std::sort(objects.begin(), objects.end(), [](
        const std::unique_ptr<GMObject> &obj1,
        const std::unique_ptr<GMObject> &obj2){
        return obj1->obj_update_order < obj2->obj_update_order;
    });

    std::vector<GMObjectMessage> messages;
    // TODO
}

void GMObjectManager::gm_draw() {
    std::sort(objects.begin(), objects.end(), [](
        const std::unique_ptr<GMObject> &obj1,
        const std::unique_ptr<GMObject> &obj2){
        return obj1->obj_draw_order < obj2->obj_draw_order;
    });

    for (auto &obj: objects) {
        if (obj->obj_visible) {
            obj->gm_draw();
        }
    }
}

void GMObjectManager::gm_add_object(GMObject new_obj) {
    for (auto &obj: objects) {
        if (obj->obj_name_id == new_obj.obj_name_id) {
            throw GMItemNameDuplicate("GMObjectManager::gm_add_object", obj->obj_name_id);
        }
    }

    objects.push_back(std::make_unique<GMObject>(new_obj));
}

void GMObjectManager::gm_remove_object(GMStringId name_id) {
    for (size_t i = 0; i < objects.size(); i++) {
        if (objects[i]->obj_name_id == name_id) {
            std::swap(objects[i], objects.back());
            objects.pop_back();
            return;
        }
    }

    throw GMItemNotFound("GMObjectManager::gm_remove_object", name_id);
}

void GMObjectManager::gm_replace_object(GMObject new_obj) {
    for (size_t i = 0; i < objects.size(); i++) {
        if (objects[i]->obj_name_id == new_obj.obj_name_id) {
            objects[i] = std::make_unique<GMObject>(new_obj);
            return;
        }
    }

    throw GMItemNotFound("GMObjectManager::gm_replace_object", new_obj.obj_name_id);
}

void GMObjectManager::gm_clear_objects() {
    objects.clear();
}

void GMObjectManager::gm_handle_message(const GMObjMgrMessage &message) {
    switch (message.msg_type) {
        case GMObjMgrMessageType::AddObject:
        {
            GMObject new_object = std::any_cast<GMObject>(message.msg_data);
            gm_add_object(new_object);
        }
        break;

        case GMObjMgrMessageType::RemoveObject:
        {
            GMStringId name_id = std::any_cast<GMStringId>(message.msg_data);
            gm_remove_object(name_id);
        }
        break;

        case GMObjMgrMessageType::ReplaceObject:
        {
            GMObject new_object = std::any_cast<GMObject>(message.msg_data);
            gm_replace_object(new_object);
        }
        break;

        case GMObjMgrMessageType::ClearObjects:
            gm_clear_objects();
        break;

        default:
            throw GMUnknownMessageType(message.msg_type);
        break;
    }
}

void GMObjectManager::gm_apply(GMStringId name_id, std::function<void(GMObject &)> fun) {
    for (auto &obj: objects) {
        if (obj->obj_name_id == name_id) {
            fun(*obj);
            return;
        }
    }

    throw GMItemNotFound("GMObjectManager::gm_apply", name_id);
}

void GMObjectManager::gm_apply_n(std::span<GMStringId> items, std::function<void(GMObject &)> fun) {
    for (auto name_id: items) {
        for (auto &obj: objects) {
            if (obj->obj_name_id == name_id) {
                fun(*obj);
                break;
            }
        }
    }
}

void GMObjectManager::gm_apply_group(GMStringId group, std::function<void(GMObject &)> fun) {
    for (auto &obj: objects) {
        if (obj->gm_is_in_group(group)) {
            fun(*obj);
        }
    }
}
}
