/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the context that is passed to the update() and draw() methods.
*/

#ifndef FILE_GM_CONTEXT_HPP_INCLUDED
#define FILE_GM_CONTEXT_HPP_INCLUDED

// STD includes:
#include <vector>

// Local includes:
#include "gm_message.hpp"

namespace gm2d {
class GMContext {
    public:
        GMContext();

        void gm_send_object_message(const GMObjectMessage &);
        void gm_send_group_message(const GMObjectMessage &);
        void gm_send_objmgr_message(const GMObjMgrMessage &);
        void gm_send_scene_message(const GMSceneMessage &);
        void gm_send_scenemgr_message(const GMSceneMgrMessage &);
        void gm_send_engine_message(const GMEngineMessage &);

        std::vector<GMObjectMessage> object_messages;
        std::vector<GMObjectMessage> group_messages;
        std::vector<GMObjMgrMessage> objmgr_messages;
        std::vector<GMSceneMessage> scene_messages;
        std::vector<GMSceneMgrMessage> scenemgr_messages;
        std::vector<GMEngineMessage> engine_messages;
};
}

#endif // FILE_GM_CONTEXT_HPP_INCLUDED
