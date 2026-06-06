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
GMMessage::GMMessage(std::string sender, std::string receiver, GMMessageSenderKind sender_kind, std::any data):
    msg_sender(sender),
    msg_receiver(receiver),
    msg_sender_kind(sender_kind),
    msg_data(data)
{}

GMNormalMessage::GMNormalMessage(std::string sender, std::string receiver, GMNormalMessageType type, GMMessageSenderKind sender_kind, std::any data):
    GMMessage(sender, receiver, sender_kind, data),
    msg_type(type)
{}

GMGFXMessage::GMGFXMessage(std::string sender, std::string receiver, GMGFXMessageType type, GMMessageSenderKind sender_kind, std::any data):
    GMMessage(sender, receiver, sender_kind, data),
    msg_type(type)
{}

GMObjMgrMessage::GMObjMgrMessage(std::string sender, std::string receiver, GMObjMgrMessageType type, GMMessageSenderKind sender_kind, std::any data):
    GMMessage(sender, receiver, sender_kind, data),
    msg_type(type)
{}

GMSceneMgrMessage::GMSceneMgrMessage(std::string sender, std::string receiver, GMSceneMgrMessageType type, GMMessageSenderKind sender_kind, std::any data):
    GMMessage(sender, receiver, sender_kind, data),
    msg_type(type)
{}
}
