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
#include "gm_string_id.hpp"

namespace gm2d {
class GMMessage {
    public:
        // Constructors:
        GMMessage(GMStringId, GMStringId, GMMessageType);
        GMMessage(GMStringId, GMStringId, GMMessageType, bool);
        GMMessage(GMStringId, GMStringId, GMMessageType, std::any);
        GMMessage(GMStringId, GMStringId, GMMessageType, bool, std::any);

        const GMStringId msg_sender;
        const GMStringId msg_receiver;
        const GMMessageType msg_type;
        const bool msg_group;
        std::any msg_data;
};
}
#endif // FILE_GM_MESSAGE_HPP_INCLUDED
