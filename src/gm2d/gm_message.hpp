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

// Local includes:
#include "gm_message_type.hpp"


namespace gm2d {
class GMMessage {
    public:
        // Constructors:
        GMMessage(GMMessageType);
        GMMessage(GMMessageType, std::any);


        const GMMessageType msg_type;
        std::any msg_data;
};

class GMHandleResult {
    public:
        // Constructors:
        GMHandleResult();
        GMHandleResult(GMHandleResultType, std::any);


        const GMHandleResultType result_type;
        std::any result_data;
};

#endif // FILE_GM_MESSAGE_HPP_INCLUDED
}
