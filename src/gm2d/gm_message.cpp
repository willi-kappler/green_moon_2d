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
GMObjectMessage::GMObjectMessage(GMStringId sender, GMStringId receiver, GMObjectMessageType type):
    msg_sender(sender),
    msg_receiver(receiver),
    msg_layer(std::nullopt),
    msg_type(type),
    msg_sender_type(GMMessageSenderType::Object),
    msg_data()
{}

GMObjectMessage::GMObjectMessage(GMStringId sender, GMStringId receiver, uint8_t layer, GMObjectMessageType type):
    msg_sender(sender),
    msg_receiver(receiver),
    msg_layer(layer),
    msg_type(type),
    msg_sender_type(GMMessageSenderType::Object),
    msg_data()
{}

[[nodiscard]] GMObjectMessage& GMObjectMessage::with_sender_type(GMMessageSenderType type) {
    msg_sender_type = type;
    return *this;
}

[[nodiscard]] GMObjectMessage& GMObjectMessage::with_msg_data(std::any data) {
    msg_data = data;
    return *this;
}

GMObjMgrMessage::GMObjMgrMessage(GMObjMgrMessageType type):
    msg_type(type),
    msg_data()
{}

GMObjMgrMessage::GMObjMgrMessage(GMObjMgrMessageType type, std::any data):
    msg_type(type),
    msg_data(data)
{}

GMSceneMessage::GMSceneMessage(GMStringId sender, GMStringId receiver, GMSceneMessageType type):
    msg_sender(sender),
    msg_receiver(receiver),
    msg_type(type),
    msg_sender_type(GMMessageSenderType::Scene),
    msg_data()
{}

[[nodiscard]] GMSceneMessage& GMSceneMessage::with_sender_type(GMMessageSenderType type) {
    msg_sender_type = type;
    return *this;
}

[[nodiscard]] GMSceneMessage& GMSceneMessage::with_msg_data(std::any data) {
    msg_data = data;
    return *this;
}

GMSceneMgrMessage::GMSceneMgrMessage(GMSceneMgrMessageType type):
    msg_type(type),
    msg_data()
{}

GMSceneMgrMessage::GMSceneMgrMessage(GMSceneMgrMessageType type, std::any data):
    msg_type(type),
    msg_data(data)
{}

GMEngineMessage::GMEngineMessage(GMEngineMessageType type):
    msg_type(type),
    msg_data()
{}

GMEngineMessage::GMEngineMessage(GMEngineMessageType type, std::any data):
    msg_type(type),
    msg_data(data)
{}

}
