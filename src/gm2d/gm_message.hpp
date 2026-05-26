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

// Messages from and to objects:
class GMMessage {
    public:
        // Constructors:
        GMMessage(std::string, std::string, GMMessageType);
        GMMessage(std::string, std::string, GMMessageType, std::any);


        const std::string msg_sender;
        const std::string msg_receiver;
        const GMMessageType msg_type;
        std::any msg_data;
};

// Messages to the object manager:
class GMOMgrMessage {
    public:
        // Constructors:
        GMOMgrMessage(GMOMgrMessageType);
        GMOMgrMessage(GMOMgrMessageType, std::any);


        const std::string msg_sender;
        const GMOMgrMessageType msg_type;
        std::any msg_data;
};

// Messages to the scene manager:
class GMSMgrMessage {
    public:
        // Constructors:
        GMSMgrMessage(GMSMgrMessageType);
        GMSMgrMessage(GMSMgrMessageType, std::any);


        const std::string msg_sender;
        const GMSMgrMessageType msg_type;
        std::any msg_data;
};



#endif // FILE_GM_MESSAGE_HPP_INCLUDED
}
