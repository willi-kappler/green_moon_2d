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
GMMessage::GMMessage(std::string sender, std::string receiver, GMMessageType type):
    GMMessage(sender, receiver, type, GMMessageCategory::Normal, {})
{}

GMMessage::GMMessage(std::string sender, std::string receiver, GMMessageType type, GMMessageCategory category, std::any data):
    msg_sender(sender),
    msg_receiver(receiver),
    msg_type(type),
    msg_category(category),
    msg_data(data)
{}
}
