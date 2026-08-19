/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the object class
*/

// STD includes:
#include <algorithm>
#include <tuple>

// Local includes:
#include "gm_object.hpp"
#include "gm_exceptions.hpp"


namespace gm2d {
GMObject::GMObject(GMStringId name_id):
    obj_name_id(name_id),
    obj_active(false),
    obj_visible(false),
    obj_update_order(0),
    obj_draw_order(0),
    obj_position(),
    obj_pos_delta(),
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


// Object manager:

GMObjectManager::GMObjectManager(uint8_t num_of_layers):
    layers()
{
    for (uint8_t i = 0; i < num_of_layers; i++) {
        layers.push_back({});
    }
}

void GMObjectManager::gm_add_layer() {
    layers.push_back({});
}

void GMObjectManager::gm_remove_layer(uint8_t index) {
    if (index < layers.size()) {
        layers.erase(layers.begin() + index);
    } else {
        throw GMLayerIndexInvalid("GMObjectManager::gm_remove_layer", index);
    }
}

void GMObjectManager::gm_clear_layer(uint8_t index) {
    if (index < layers.size()) {
        layers[index].clear();
    } else {
        throw GMLayerIndexInvalid("GMObjectManager::gm_clear_layer", index);
    }
}

void GMObjectManager::gm_clear_all_layers() {
    layers.clear();
}

void GMObjectManager::gm_update(GMContext &context) {
    // Need to move to local variable, since gm_handle_message could add more messages to context.
    std::vector<GMObjMgrMessage> objmgr_messages = context.gm_get_objmgr_messages();

    for (auto &message: objmgr_messages) {
        gm_handle_message(message);
    }

    objmgr_messages.clear();

    std::vector<GMObjectMessage> object_messages = context.gm_get_object_messages();
    bool object_found;
    uint8_t layer_index;

    for (auto &message: object_messages) {
        object_found = false;

        if (message.msg_layer.has_value()) {
            layer_index = *message.msg_layer;
            if (layer_index < layers.size()) {
                for (auto &object: layers[layer_index]) {
                    if (message.msg_receiver == object->obj_name_id) {
                        object->gm_handle_message(message, context);
                        object_found = true;
                        break;
                    }
                }
            } else {
                throw GMLayerIndexInvalid("GMObjectManager::gm_update", layer_index);
            }
        } else {
            // No layer specified, look for the object name in all layers.
            for (auto &layer: layers) {
                for (auto &object: layer) {
                    if (message.msg_receiver == object->obj_name_id) {
                        object->gm_handle_message(message, context);
                        object_found = true;
                        break;
                    }
                }

                if (object_found) {
                    break;
                }
            }
        }

        if (!object_found) {
            throw GMItemNotFound("GMObjectManager::gm_update", message.msg_receiver);
        }
    }

    object_messages.clear();

    std::vector<GMObjectMessage> group_messages = context.gm_get_group_messages();

    for (auto &message: group_messages) {
        for (auto &layer: layers) {
            for (auto &object: layer) {
                if (object->gm_is_in_group(message.msg_receiver)) {
                    object->gm_handle_message(message, context);
                }
            }
        }
    }

    group_messages.clear();

    for (auto &layer: layers) {
        for (auto &object: layer) {
            if (object->obj_active) {
                object->gm_update(context);
            }
        }
    }
}

void GMObjectManager::gm_draw(GMContext &context) {
    for (auto &layer: layers) {
        for (auto &object: layer) {
            if (object->obj_visible) {
                object->gm_draw(context);
            }
        }
    }
}

void GMObjectManager::gm_add_object(std::shared_ptr<GMObject> new_obj, uint8_t layer_index) {
    if (layer_index < layers.size()) {
        auto &layer = layers[layer_index];

        for (auto &object: layer) {
            if (object->obj_name_id == new_obj->obj_name_id) {
                throw GMItemNameDuplicate("GMObjectManager::gm_add_object", object->obj_name_id);
            }
        }

        layer.push_back(new_obj);
    } else {
        throw GMLayerIndexInvalid("GMObjectManager::gm_add_object", layer_index);
    }
}

void GMObjectManager::gm_remove_object(GMStringId name_id, uint8_t layer_index) {
    if (layer_index < layers.size()) {
        auto &layer = layers[layer_index];

        for (size_t i = 0; i < layer.size(); i++) {
            if (layer[i]->obj_name_id == name_id) {
                std::swap(layer[i], layer.back());
                layer.pop_back();
                return;
            }
        }

        throw GMItemNotFound("GMObjectManager::gm_remove_object", name_id);
    } else {
        throw GMLayerIndexInvalid("GMObjectManager::gm_remove_object", layer_index);
    }
}

void GMObjectManager::gm_replace_object(std::shared_ptr<GMObject> new_obj, uint8_t layer_index) {
    if (layer_index < layers.size()) {
        auto &layer = layers[layer_index];

        for (size_t i = 0; i < layer.size(); i++) {
            if (layer[i]->obj_name_id == new_obj->obj_name_id) {
                layer[i] = new_obj;
                return;
            }
        }

        throw GMItemNotFound("GMObjectManager::gm_replace_object", new_obj->obj_name_id);
    } else {
        throw GMLayerIndexInvalid("GMObjectManager::gm_replace_object", layer_index);
    }
}

void GMObjectManager::gm_clear_all_objects() {
    for (auto &layer: layers) {
        layer.clear();
    }
}

void GMObjectManager::gm_move_to_layer1(GMStringId name_id, uint8_t dst_layer) {
    if (dst_layer < layers.size()) {
        uint8_t index;

        for (auto &layer: layers) {
            for (index = 0; index < layer.size(); index++) {
                if (layer[index]->obj_name_id == name_id) {
                    layers[dst_layer].push_back(layer[index]);
                    std::swap(layer[index], layer.back());
                    layer.pop_back();
                    return;
                }
            }
        }

        throw GMItemNotFound("GMObjectManager::gm_move_to_layer", name_id);
    } else {
        throw GMLayerIndexInvalid("GMObjectManager::gm_move_to_layer: dst_layer", dst_layer);
    }
}

void GMObjectManager::gm_move_to_layer2(GMStringId name_id, uint8_t src_layer, uint8_t dst_layer) {
    if (src_layer < layers.size()) {
        if (dst_layer < layers.size()) {
            uint8_t index;
            auto &layer = layers[src_layer];

            for (index = 0; index < layer.size(); index++) {
                if (layer[index]->obj_name_id == name_id) {
                    layers[dst_layer].push_back(layer[index]);
                    std::swap(layer[index], layer.back());
                    layer.pop_back();
                    return;
                }
            }

            throw GMItemNotFound("GMObjectManager::gm_move_to_layer", name_id);
        } else {
            throw GMLayerIndexInvalid("GMObjectManager::gm_move_to_layer: dst_layer", dst_layer);
        }
    } else {
        throw GMLayerIndexInvalid("GMObjectManager::gm_move_to_layer: src_layer", src_layer);
    }
}

void GMObjectManager::gm_handle_message(GMObjMgrMessage &message) {
    switch (message.msg_type) {
        case GMObjMgrMessageType::AddLayer:
        {
            gm_add_layer();
        }
        break;

        case GMObjMgrMessageType::RemoveLayer:
        {
            uint8_t layer_index = std::any_cast<uint8_t>(message.msg_data);
            gm_remove_layer(layer_index);
        }
        break;

        case GMObjMgrMessageType::ClearLayer:
        {
            uint8_t layer_index = std::any_cast<uint8_t>(message.msg_data);
            gm_clear_layer(layer_index);
        }
        break;

        case GMObjMgrMessageType::ClearAllLayers:
        {
            gm_clear_all_layers();
        }
        break;

        case GMObjMgrMessageType::AddObject:
        {
            auto [new_object, layer_index] = std::any_cast<std::tuple<std::shared_ptr<GMObject>, uint8_t>>(message.msg_data);
            gm_add_object(new_object, layer_index);
        }
        break;

        case GMObjMgrMessageType::RemoveObject:
        {
            auto [name_id, layer_index] = std::any_cast<std::tuple<GMStringId, uint8_t>>(message.msg_data);
            gm_remove_object(name_id, layer_index);
        }
        break;

        case GMObjMgrMessageType::ReplaceObject:
        {
            auto [new_object, layer_index] = std::any_cast<std::tuple<std::shared_ptr<GMObject>, uint8_t>>(message.msg_data);
            gm_replace_object(new_object, layer_index);
        }
        break;

        case GMObjMgrMessageType::ClearAllObjects:
        {
            gm_clear_all_objects();
        }
        break;

        case GMObjMgrMessageType::MoveToLayer1:
        {
            auto [name_id, dst_layer] = std::any_cast<std::tuple<GMStringId, uint8_t>>(message.msg_data);
            gm_move_to_layer1(name_id, dst_layer);
        }
        break;

        case GMObjMgrMessageType::MoveToLayer2:
        {
            auto [name_id, src_layer, dst_layer] = std::any_cast<std::tuple<GMStringId, uint8_t, uint8_t>>(message.msg_data);
            gm_move_to_layer2(name_id, src_layer, dst_layer);
        }
        break;

        default:
            throw GMUnknownMessageType(message.msg_type);
        break;
    }
}

void GMObjectManager::gm_apply(GMStringId name_id, std::function<void(GMObject &)> fun, uint8_t layer_index) {
    if (layer_index < layers.size()) {
        for (auto &object: layers[layer_index]) {
            if (object->obj_name_id == name_id) {
                fun(*object);
                return;
            }
        }

        throw GMItemNotFound("GMObjectManager::gm_apply", name_id);
    } else {
        throw GMLayerIndexInvalid("GMObjectManager::gm_apply", layer_index);
    }
}

void GMObjectManager::gm_apply_n(std::span<GMStringId> items, std::function<void(GMObject &)> fun, uint8_t layer_index) {
    if (layer_index < layers.size()) {
        bool object_found;

        // First iterate through all names, then all objects.
        // If a name was not found -> throw an exception.
        for (auto name_id: items) {
            object_found = false;

            for (auto &object: layers[layer_index]) {
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
    } else {
        throw GMLayerIndexInvalid("GMObjectManager::gm_apply_n", layer_index);
    }
}

void GMObjectManager::gm_apply_n(std::span<GMStringId> items, std::function<void(GMObject &)> fun) {
    bool object_found;

    // First iterate through all names, then all objects.
    // If a name was not found -> throw an exception.
    for (auto name_id: items) {
        object_found = false;

        for (auto &layer: layers) {
            for (auto &object: layer) {
                if (object->obj_name_id == name_id) {
                    fun(*object);
                    object_found = true;
                    break;
                }
            }

            if (object_found) {
                break;
            }
        }

        if (!object_found) {
            throw GMItemNotFound("GMObjectManager::gm_apply_n", name_id);
        }
    }
}

void GMObjectManager::gm_apply_group(GMStringId group, std::function<void(GMObject &)> fun, uint8_t layer_index) {
    if (layer_index < layers.size()) {
        for (auto &object: layers[layer_index]) {
            if (object->gm_is_in_group(group)) {
                fun(*object);
            }
        }
    } else {
        throw GMLayerIndexInvalid("GMObjectManager::gm_apply_group", layer_index);
    }
}

void GMObjectManager::gm_apply_group(GMStringId group, std::function<void(GMObject &)> fun) {
    for (auto &layer: layers) {
        for (auto &object: layer) {
            if (object->gm_is_in_group(group)) {
                fun(*object);
            }
        }
    }
}
}
