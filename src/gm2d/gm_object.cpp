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

[[nodiscard]] GMObject& GMObject::with_active(bool active) {
    obj_active = active;
    return *this;
}

[[nodiscard]] GMObject& GMObject::with_visible(bool visible) {
    obj_visible = visible;
    return *this;
}

[[nodiscard]] GMObject& GMObject::with_update_order(int16_t update_order) {
    obj_update_order = update_order;
    return *this;
}

[[nodiscard]] GMObject& GMObject::with_draw_order(int16_t draw_order) {
    obj_draw_order = draw_order;
    return *this;
}

[[nodiscard]] GMObject& GMObject::with_position(GMVec2D position) {
    obj_position = position;
    return *this;
}

void GMObject::gm_handle_message(const GMObjectMessage &message, GMContext &context) {
    switch (message.msg_type) {
        case GMObjectMessageType::SetActive:
            obj_active = std::any_cast<bool>(message.msg_data);
        break;

        case GMObjectMessageType::GetActive:
        {
            GMObjectMessage reply_message = GMObjectMessage(obj_name_id, message.msg_sender, GMObjectMessageType::GetActiveResult)
                .with_msg_data(obj_active);
            context.gm_send_object_message(reply_message);
        }
        break;

        case GMObjectMessageType::ToggleActive:
            obj_active = !obj_active;
        break;

        case GMObjectMessageType::SetUpdateOrder:
            obj_update_order = std::any_cast<int16_t>(message.msg_data);
        break;

        case GMObjectMessageType::GetUpdateOrder:
        {
            GMObjectMessage reply_message = GMObjectMessage(obj_name_id, message.msg_sender, GMObjectMessageType::GetUpdateOrderResult)
                .with_msg_data(obj_update_order);
            context.gm_send_object_message(reply_message);
        }
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
        {
            GMObjectMessage reply_message = GMObjectMessage(obj_name_id, message.msg_sender, GMObjectMessageType::GetVisibleResult)
                .with_msg_data(obj_visible);
            context.gm_send_object_message(reply_message);
        }
        break;

        case GMObjectMessageType::ToggleVisible:
            obj_visible = !obj_visible;
        break;

        case GMObjectMessageType::SetDrawOrder:
            obj_draw_order = std::any_cast<int16_t>(message.msg_data);
        break;

        case GMObjectMessageType::GetDrawOrder:
        {
            GMObjectMessage reply_message = GMObjectMessage(obj_name_id, message.msg_sender, GMObjectMessageType::GetVisibleResult)
                .with_msg_data(obj_draw_order);
            context.gm_send_object_message(reply_message);
        }
        break;

        case GMObjectMessageType::SetPosition:
            obj_position = std::any_cast<GMVec2D>(message.msg_data);
        break;

        case GMObjectMessageType::GetPosition:
        {
            GMObjectMessage reply_message = GMObjectMessage(obj_name_id, message.msg_sender, GMObjectMessageType::GetPositionResult)
                .with_msg_data(obj_position);
            context.gm_send_object_message(reply_message);
        }
        break;

        case GMObjectMessageType::AddPosition:
            obj_position += std::any_cast<GMVec2D>(message.msg_data);
        break;

        default:
            throw GMUnknownMessageType(message.msg_type, obj_name_id);
        break;
    }
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

void GMObjectManager::gm_update(GMContext &context) {
    std::vector<GMObjMgrMessage> objmgr_messages = std::move(context.objmgr_messages);

    for (auto &message: objmgr_messages) {
        gm_handle_message(message);
    }

    objmgr_messages.clear();

    std::vector<GMObjectMessage> object_messages = std::move(context.object_messages);
    bool object_found;

    for (auto &message: object_messages) {
        object_found = false;

        for (auto &object: objects) {
            if (message.msg_receiver == object->obj_name_id) {
                object->gm_handle_message(message, context);
                object_found = true;
                break;
            }
        }

        if (!object_found) {
            throw GMItemNotFound("GMObjectManager::gm_update", message.msg_receiver);
        }
    }

    object_messages.clear();

    std::vector<GMObjectMessage> group_messages = std::move(context.group_messages);

    for (auto &message: group_messages) {
        for (auto &object: objects) {
            if (object->gm_is_in_group(message.msg_receiver)) {
                object->gm_handle_message(message, context);
            }
        }
    }

    group_messages.clear();

    std::sort(objects.begin(), objects.end(), [](
        const std::unique_ptr<GMObject> &obj1,
        const std::unique_ptr<GMObject> &obj2){
        return obj1->obj_update_order < obj2->obj_update_order;
    });

    for (auto &object: objects) {
        if (object->obj_active) {
            object->gm_update(context);
        }
    }
}

void GMObjectManager::gm_draw(GMContext &context) {
    std::sort(objects.begin(), objects.end(), [](
        const std::unique_ptr<GMObject> &object1,
        const std::unique_ptr<GMObject> &object2){
        return object1->obj_draw_order < object2->obj_draw_order;
    });

    for (auto &object: objects) {
        if (object->obj_visible) {
            object->gm_draw(context);
        }
    }
}

void GMObjectManager::gm_add_object(std::unique_ptr<GMObject> new_obj) {
    for (auto &object: objects) {
        if (object->obj_name_id == new_obj->obj_name_id) {
            throw GMItemNameDuplicate("GMObjectManager::gm_add_object", object->obj_name_id);
        }
    }

    objects.push_back(std::move(new_obj));
}

/*
void GMObjectManager::gm_add_object(const GMObject &new_obj) {
    gm_add_object(std::make_unique<GMObject>(new_obj));
}
*/

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

void GMObjectManager::gm_replace_object(std::unique_ptr<GMObject> new_obj) {
    for (size_t i = 0; i < objects.size(); i++) {
        if (objects[i]->obj_name_id == new_obj->obj_name_id) {
            objects[i] = std::move(new_obj);
            return;
        }
    }

    throw GMItemNotFound("GMObjectManager::gm_replace_object", new_obj->obj_name_id);
}

/*
void GMObjectManager::gm_replace_object(const GMObject &new_obj) {
    gm_replace_object(std::make_unique<GMObject>(new_obj));
}
*/

void GMObjectManager::gm_clear_objects() {
    objects.clear();
}

void GMObjectManager::gm_handle_message(GMObjMgrMessage &message) {
    switch (message.msg_type) {
        case GMObjMgrMessageType::AddObject:
        {
            // auto new_obj = std::any_cast<GMObject *>(message.msg_data);
            /*
            if (auto* obj_ptr = std::any_cast<std::unique_ptr<GMObject>>(&message.msg_data)) {
                gm_add_object(static_cast<std::unique_ptr<GMObject>&&>(*obj_ptr));
            }
            */
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
            //gm_replace_object(std::any_cast<std::unique_ptr<GMObject>&&>(message.msg_data));
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
    for (auto &object: objects) {
        if (object->obj_name_id == name_id) {
            fun(*object);
            return;
        }
    }

    throw GMItemNotFound("GMObjectManager::gm_apply", name_id);
}

void GMObjectManager::gm_apply_n(std::span<GMStringId> items, std::function<void(GMObject &)> fun) {
    bool object_found;

    for (auto name_id: items) {
        object_found = false;

        for (auto &object: objects) {
            if (object->obj_name_id == name_id) {
                fun(*object);
                object_found = true;
                break;
            }
        }

        if (!object_found) {
            throw GMItemNotFound("GMObjectManager::gm_apply_n", name_id);
        }
    }
}

void GMObjectManager::gm_apply_group(GMStringId group, std::function<void(GMObject &)> fun) {
    for (auto &object: objects) {
        if (object->gm_is_in_group(group)) {
            fun(*object);
        }
    }
}
}
