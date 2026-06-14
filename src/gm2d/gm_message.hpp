/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the message class
*/

#ifndef FILE_GM_MESSAGE_HPP_INCLUDED
#define FILE_GM_MESSAGE_HPP_INCLUDED

// STD includes:
#include <cstdint>
#include <any>
// #include <string>

// Local includes:
#include "gm_message_type.hpp"
#include "gm_string_id.hpp"

namespace gm2d {
class GMObjectMessage {
    public:
        // Constructors:
        GMObjectMessage(GMStringId, GMStringId, GMObjectMessageType);

        // Fluent builder pattern:
        [[nodiscard]] GMObjectMessage& with_sender_type(GMMessageSenderType);
        [[nodiscard]] GMObjectMessage& with_msg_data(std::any);

        const GMStringId msg_sender;
        const GMStringId msg_receiver;
        const GMObjectMessageType msg_type;
        GMMessageSenderType msg_sender_type;
        std::any msg_data;
};

class GMObjMgrMessage {
    public:
        // Constructors:
        GMObjMgrMessage(GMObjMgrMessageType);
        GMObjMgrMessage(GMObjMgrMessageType, std::any);

        const GMObjMgrMessageType msg_type;
        std::any msg_data;
};

class GMSceneMessage {
    public:
        // Constructors:
        GMSceneMessage(GMStringId, GMStringId, GMSceneMessageType);

        // Fluent builder pattern:
        [[nodiscard]] GMSceneMessage& with_sender_type(GMMessageSenderType);
        [[nodiscard]] GMSceneMessage& with_msg_data(std::any);

        const GMStringId msg_sender;
        const GMStringId msg_receiver;
        const GMSceneMessageType msg_type;
        GMMessageSenderType msg_sender_type;
        std::any msg_data;
};

class GMSceneMgrMessage {
    public:
        // Constructors:
        GMSceneMgrMessage(GMSceneMgrMessageType);
        GMSceneMgrMessage(GMSceneMgrMessageType, std::any);

        const GMSceneMgrMessageType msg_type;
        std::any msg_data;
};

class GMEngineMessage {
    public:
        // Constructors:
        GMEngineMessage(GMEngineMessageType);
        GMEngineMessage(GMEngineMessageType, std::any);

        const GMEngineMessageType msg_type;
        std::any msg_data;
};

}
#endif // FILE_GM_MESSAGE_HPP_INCLUDED
