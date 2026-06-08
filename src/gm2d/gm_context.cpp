/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the context that is passed to the update() and draw() methods.
*/


// Local includes:
#include "gm_context.hpp"

namespace gm2d {
GMContext::GMContext():
    object_messages(),
    group_messages(),
    objmgr_messages(),
    scene_messages(),
    scenemgr_messages(),
    engine_messages()
{}

void GMContext::gm_send_object_message(const GMObjectMessage &message) {
    object_messages.push_back(message);
}

void GMContext::gm_send_group_message(const GMObjectMessage &message) {
    group_messages.push_back(message);
}

void GMContext::gm_send_objmgr_message(const GMObjMgrMessage &message) {
    objmgr_messages.push_back(message);
}

void GMContext::gm_send_scene_message(const GMSceneMessage &message) {
    scene_messages.push_back(message);
}

void GMContext::gm_send_scenemgr_message(const GMSceneMgrMessage &message) {
    scenemgr_messages.push_back(message);
}

void GMContext::gm_send_engine_message(const GMEngineMessage &message) {
    engine_messages.push_back(message);
}
}
