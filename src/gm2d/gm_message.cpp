/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the message class
*/

// Local includes:
#include "gm_message.hpp"

namespace gm2d {
GMMessage::GMMessage(GMMessageType type):
    msg_type(type),
    msg_data()
{}

GMMessage::GMMessage(GMMessageType type, std::any data):
    msg_type(type),
    msg_data(data)
{}

GMHandleResult::GMHandleResult():
    result_type(GMHandleResultType::Empty),
    result_data()
{}

GMHandleResult::GMHandleResult(GMHandleResultType type, std::any data):
    result_type(type),
    result_data(data)
{}
}
