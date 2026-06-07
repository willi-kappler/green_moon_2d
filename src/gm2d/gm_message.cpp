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
GMMessage::GMMessage(GMStringId sender, GMStringId receiver, GMMessageType type):
    GMMessage(sender, receiver, type, false, {})
{}

GMMessage::GMMessage(GMStringId sender, GMStringId receiver, GMMessageType type, bool group_message):
    GMMessage(sender, receiver, type, group_message, {})
{}

GMMessage::GMMessage(GMStringId sender, GMStringId receiver, GMMessageType type, std::any data):
    GMMessage(sender, receiver, type, false, data)
{}

GMMessage::GMMessage(GMStringId sender, GMStringId receiver, GMMessageType type, bool group_message, std::any data):
    msg_sender(sender),
    msg_receiver(receiver),
    msg_type(type),
    msg_group(group_message),
    msg_data(data)
{}
}
