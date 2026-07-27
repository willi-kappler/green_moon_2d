/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the scene manager
*/

// Local includes:
#include "gm_scene.hpp"
#include "gm_exceptions.hpp"

namespace gm2d {
GMScene::GMScene(GMStringId scene_id):
    name_id(scene_id),
    on_stack(false)
{}


// Scene manager:

GMSceneManager::GMSceneManager():
    scenes(),
    current_scene(),
    scene_stack()
{}

void GMSceneManager::gm_update(GMContext &context) {
    std::vector<GMSceneMgrMessage> scenemgr_messages = std::move(context.scenemgr_messages);

    for (auto &message: scenemgr_messages) {
        gm_handle_message(message, context);
    }

    scenemgr_messages.clear();

    std::vector<GMSceneMessage> scene_messages = std::move(context.scene_messages);
    bool scene_found;

    for (auto &message: scene_messages) {
        scene_found = false;

        for (auto &scene: scenes) {
            if (message.msg_receiver == scene->name_id) {
                scene->gm_handle_message(message, context);
                scene_found = true;
                break;
            }
        }

        if (!scene_found) {
            throw GMItemNotFound("GMSceneManager::gm_update", message.msg_receiver);
        }
    }

    scene_messages.clear();

    current_scene->gm_update(context);
}

void GMSceneManager::gm_draw(GMContext &context) {
    current_scene->gm_draw(context);
}

void GMSceneManager::gm_add_scene(std::shared_ptr<GMScene> new_scene) {
    for (auto &scene: scenes) {
        if (scene->name_id == new_scene->name_id) {
            throw GMItemNameDuplicate("GMSceneManager::gm_add_scene", new_scene->name_id);
        }
    }

    scenes.push_back(new_scene);
}

void GMSceneManager::gm_remove_scene(GMStringId name_id) {
    for (size_t i = 0; i < scenes.size(); i++) {
        if (scenes[i]->name_id == name_id) {
            std::swap(scenes[i], scenes.back());
            scenes.pop_back();
            return;
        }
    }

    throw GMItemNotFound("GMSceneManager::gm_delete_scene", name_id);
}

void GMSceneManager::gm_replace_scene(std::shared_ptr<GMScene> new_scene) {
    for (size_t i = 0; i < scenes.size(); i++) {
        if (scenes[i]->name_id == new_scene->name_id) {
            scenes[i] = new_scene;
            return;
        }
    }

    throw GMItemNotFound("GMSceneManager::gm_replace_scene", new_scene->name_id);
}

void GMSceneManager::gm_change_to_scene(GMStringId name_id) {
    for (auto &scene: scenes) {
        if (scene->name_id == name_id) {
            GMStringId prev_name_id = current_scene->name_id;
            current_scene = scene;
            current_scene->gm_enter(prev_name_id);
            return;
        }
    }

    throw GMItemNotFound("GMSceneManager::gm_change_to_scene", name_id);
}

void GMSceneManager::gm_push_and_change(GMStringId name_id) {
    for (auto &scene: scenes) {
        if (scene->name_id == name_id) {
            current_scene->on_stack = true;
            scene_stack.push_back(current_scene);
            GMStringId prev_name_id = current_scene->name_id;
            current_scene = scene;
            current_scene->gm_enter(prev_name_id);
            return;
        }
    }

    throw GMItemNotFound("GMSceneManager::gm_push_and_change", name_id);
}

void GMSceneManager::gm_pop_and_change() {
    GMStringId prev_name_id = current_scene->name_id;
    current_scene = scene_stack.back();
    current_scene->on_stack = false;
    scene_stack.pop_back();
    current_scene->gm_enter(prev_name_id);
}

void GMSceneManager::gm_set_start_scene(GMStringId name_id) {
    for (auto &scene: scenes) {
        if (scene->name_id == name_id) {
            current_scene = scene;
            current_scene->gm_enter(GMID(""));
            return;
        }
    }

    throw GMItemNotFound("GMSceneManager::gm_set_start_scene", name_id);
}

void GMSceneManager::gm_update_stack_top(GMContext &context) {
    scenes.back()->gm_update(context);
}

void GMSceneManager::gm_draw_stack_top(GMContext &context) {
    scenes.back()->gm_draw(context);
}

void GMSceneManager::gm_update_scene(GMStringId name_id, GMContext &context) {
    for (auto &scene: scenes) {
        if (scene->name_id == name_id) {
            scene->gm_update(context);
            return;
        }
    }

    throw GMItemNotFound("GMSceneManager::gm_update_scene", name_id);
}

void GMSceneManager::gm_draw_scene(GMStringId name_id, GMContext &context) {
    for (auto &scene: scenes) {
        if (scene->name_id == name_id) {
            scene->gm_draw(context);
            return;
        }
    }

    throw GMItemNotFound("GMSceneManager::gm_draw_scene", name_id);
}

void GMSceneManager::gm_handle_message(const GMSceneMgrMessage &message, GMContext &context) {
    switch (message.msg_type) {
        case GMSceneMgrMessageType::AddScene:
        {
            std::shared_ptr<GMScene> new_scene = std::any_cast<std::shared_ptr<GMScene>>(message.msg_data);
            gm_add_scene(new_scene);
        }
        break;

        case GMSceneMgrMessageType::RemoveScene:
        {
            GMStringId name_id = std::any_cast<GMStringId>(message.msg_data);
            gm_remove_scene(name_id);
        }
        break;

        case GMSceneMgrMessageType::ReplaceScene:
        {
            std::shared_ptr<GMScene> new_scene = std::any_cast<std::shared_ptr<GMScene>>(message.msg_data);
            gm_replace_scene(new_scene);
        }
        break;

        case GMSceneMgrMessageType::ChangeToScene:
        {
            GMStringId name_id = std::any_cast<GMStringId>(message.msg_data);
            gm_change_to_scene(name_id);
        }
        break;

        case GMSceneMgrMessageType::PushAndChange:
        {
            GMStringId name_id = std::any_cast<GMStringId>(message.msg_data);
            gm_push_and_change(name_id);
        }
        break;

        case GMSceneMgrMessageType::PopAndChange:
            gm_pop_and_change();
        break;

        case GMSceneMgrMessageType::UpdateStackTop:
            gm_update_stack_top(context);
        break;

        case GMSceneMgrMessageType::DrawStackTop:
            gm_draw_stack_top(context);
        break;

        case GMSceneMgrMessageType::UpdateScene:
        {
            GMStringId name_id = std::any_cast<GMStringId>(message.msg_data);
            gm_update_scene(name_id, context);
        }
        break;

        case GMSceneMgrMessageType::DrawScene:
        {
            GMStringId name_id = std::any_cast<GMStringId>(message.msg_data);
            gm_draw_scene(name_id, context);
        }
        break;

        default:
            throw GMUnknownMessageType(message.msg_type);
        break;
    }
}
}
