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
    obj_messages(),
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

void GMObject::gm_handle_message(const GMMessage &message) {
    switch (message.msg_type) {
        case GMMessageType::SetActive:
            obj_active = std::any_cast<bool>(message.msg_data);
        break;

        case GMMessageType::GetActive:
            //return GMHandleResult(obj_name, message.msg_sender, GMHandleResultType::Active, obj_active);
        break;

        case GMMessageType::ToggleActive:
            obj_active = !obj_active;
        break;

        case GMMessageType::SetUpdateOrder:
            obj_update_order = std::any_cast<int16_t>(message.msg_data);
        break;

        case GMMessageType::GetUpdateOrder:
            //return GMHandleResult(obj_name, message.msg_sender, GMHandleResultType::UpdateOrder, obj_update_order);
        break;

        case GMMessageType::AddGroup:
            gm_add_group(std::any_cast<GMStringId>(message.msg_data));
        break;

        case GMMessageType::RemoveGroup:
            gm_remove_group(std::any_cast<GMStringId>(message.msg_data));
        break;

        case GMMessageType::ClearGroups:
            gm_clear_groups();
        break;

        case GMMessageType::SetVisible:
            obj_visible = std::any_cast<bool>(message.msg_data);
        break;

        case GMMessageType::GetVisible:
            // TODO
        break;

        case GMMessageType::ToggleVisible:
            obj_visible = !obj_visible;
        break;

        case GMMessageType::SetDrawOrder:
            obj_draw_order = std::any_cast<int16_t>(message.msg_data);
        break;

        case GMMessageType::GetDrawOrder:
            // TODO
        break;

        case GMMessageType::SetPosition:
            obj_position = std::any_cast<GMVec2D>(message.msg_data);
        break;

        case GMMessageType::GetPosition:
            // TODO
        break;

        case GMMessageType::AddPosition:
            obj_position += std::any_cast<GMVec2D>(message.msg_data);
        break;

        default:
            throw GMUnknownMessageType(message.msg_type, obj_name_id);
        break;
    }
}

void GMObject::gm_send_message(const GMMessage message) {
    obj_messages.push_back(message);
}

void GMObject::gm_update() {
    // TODO: throw exception
}

void GMObject::gm_draw() {
    // TODO: throw exception
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

    std::vector<GMMessage> messages;

    for (auto &obj: objects) {
        if (obj->obj_active) {
            obj->gm_update();
        }

        // Process outgoing messages from current object:
        messages = std::move(obj->obj_messages);

        for (auto &message: messages) {
            gm_send_message(message);
        }
    }
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
            throw GMObjectNameDuplicate("GMObjectManager::gm_add_object", obj->obj_name_id);
        }
    }

    objects.push_back(std::make_unique<GMObject>(new_obj));
}

void GMObjectManager::gm_remove_object(GMStringId name_id) {
    for (size_t i = 0; i < objects.size(); i++) {
        if (objects[i]->obj_name_id == name_id) {
            // TODO: remove object
            return;
        }
    }

    throw GMObjectNotFound("GMObjectManager::gm_remove_object", name_id);
}

void GMObjectManager::gm_replace_object(GMStringId name_id, GMObject new_obj) {
    for (size_t i = 0; i < objects.size(); i++) {
        if (objects[i]->obj_name_id == name_id) {
            objects[i] = std::make_unique<GMObject>(new_obj);
            return;
        }
    }

    throw GMObjectNotFound("GMObjectManager::gm_replace_object", name_id);
}

void GMObjectManager::gm_clear_objects() {
    objects.clear();
}

void GMObjectManager::gm_send_message(const GMMessage &message) {
    if (message.msg_receiver == GMID("ObjectManager")) {
        gm_handle_message(message);
    } else if (message.msg_receiver == GMID("SceneManager")) {
        scene_messages.push_back(message);
    } else {
        if (message.msg_group) {
            for (auto &obj: objects) {
                if (obj->gm_is_in_group(message.msg_receiver)) {
                    obj->gm_handle_message(message);
                }
            }
        } else {
            for (auto &obj: objects) {
                if (obj->obj_name_id == message.msg_receiver) {
                    obj->gm_send_message(message);
                    return;
                }
            }

            throw GMObjectNotFound("GMObjectManager::gm_send_message", message.msg_receiver);
        }
    }
}

void GMObjectManager::gm_handle_message(const GMMessage &message) {
    if (message.msg_receiver == GMID("ObjectManager")) {
        switch (message.msg_type) {
            case GMMessageType::AddObject:
            {
                GMObject new_object = std::any_cast<GMObject>(message.msg_data);
                gm_add_object(new_object);
            }
            break;

            case GMMessageType::RemoveObject:
            {
                GMStringId name_id = std::any_cast<GMStringId>(message.msg_data);
                gm_remove_object(name_id);
            }
            break;

            case GMMessageType::ReplaceObject:
            {
                auto [name_id, new_object] = std::any_cast<std::pair<GMStringId, GMObject>>(message.msg_data);
                gm_replace_object(name_id, new_object);
            }
            break;

            case GMMessageType::ClearObjects:
                gm_clear_objects();
            break;

            default:
                throw GMUnknownMessageType(message.msg_type, GMID("ObjectManager"));
            break;
        }
    } else {
        throw GMInvalidReceiver("GMObjectManager::gm_handle_message", message.msg_receiver);
    }
}

void GMObjectManager::gm_apply(GMStringId name_id, std::function<void(GMObject &)> fun) {
    for (auto &obj: objects) {
        if (obj->obj_name_id == name_id) {
            fun(*obj);
            return;
        }
    }

    throw GMObjectNotFound("GMObjectManager::gm_apply", name_id);
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
