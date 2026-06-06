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
#include <string>

// Local includes:
#include "gm_message_type.hpp"


namespace gm2d {
class GMMessage {
    public:
        // Constructors:
        GMMessage(std::string, std::string, GMMessageSenderKind, std::any);

        const std::string msg_sender;
        const std::string msg_receiver;
        const GMMessageSenderKind msg_sender_kind;
        std::any msg_data;
};

class GMNormalMessage: public GMMessage {
    public:
        // Constructors:
        GMNormalMessage(std::string, std::string, GMNormalMessageType, GMMessageSenderKind, std::any);

        const GMNormalMessageType msg_type;
};

class GMGFXMessage: public GMMessage {
    public:
        // Constructors:
        GMGFXMessage(std::string, std::string, GMGFXMessageType, GMMessageSenderKind, std::any);

        const GMGFXMessageType msg_type;
};

class GMObjMgrMessage: public GMMessage {
    public:
        // Constructors:
        GMObjMgrMessage(std::string, std::string, GMObjMgrMessageType, GMMessageSenderKind, std::any);

        const GMObjMgrMessageType msg_type;
};

class GMSceneMgrMessage: public GMMessage {
    public:
        // Constructors:
        GMSceneMgrMessage(std::string, std::string, GMSceneMgrMessageType, GMMessageSenderKind, std::any);

        const GMSceneMgrMessageType msg_type;
};
}
#endif // FILE_GM_MESSAGE_HPP_INCLUDED
