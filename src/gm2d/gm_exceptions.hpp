/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the message class
*/

#ifndef FILE_GM_EXCEPTIONS_HPP_INCLUDED
#define FILE_GM_EXCEPTIONS_HPP_INCLUDED

// STD includes:
#include <stdexcept>
#include <format>
#include <string_view>
#include <cstdint>

// Local includes:
#include "gm_message_type.hpp"
#include "gm_string_id.hpp"

namespace gm2d {
// General:
class GMItemNotFound: public std::runtime_error {
public:
  GMItemNotFound(std::string_view where, GMStringId item_name):
    std::runtime_error(std::format("Item not found ({}): {}", where, item_name)) { }
};

class GMItemNameDuplicate: public std::runtime_error {
public:
  GMItemNameDuplicate(std::string_view where, GMStringId item_name):
    std::runtime_error(std::format("Item already exists ({}): {}", where, item_name)) { }
};

class GMMethodNotImplemented: public std::runtime_error {
public:
  GMMethodNotImplemented(std::string_view where, GMStringId obj_name):
    std::runtime_error(std::format("Method not implemented ({}): {}", where, obj_name)) { }
};

/*
class GMInvalidReceiver: public std::runtime_error {
public:
  GMInvalidReceiver(std::string_view where, GMStringId receiver):
    std::runtime_error(std::format("Invalid receiver ({}): {}", where, receiver.value)) { }
};
*/

// Messages:
class GMUnknownMessageType: public std::runtime_error {
public:
  GMUnknownMessageType(GMObjectMessageType msg_type, GMStringId obj_name):
    std::runtime_error(std::format("Unknown object message type: {}, object id: {}", msg_type, obj_name.value)) { }
  GMUnknownMessageType(GMObjMgrMessageType msg_type):
    std::runtime_error(std::format("Unknown object manager message type: {}", msg_type)) { }
  GMUnknownMessageType(GMSceneMessageType msg_type, GMStringId scene_name):
    std::runtime_error(std::format("Unknown scene message type: {}, scene id: {}", msg_type, scene_name.value)) { }
  GMUnknownMessageType(GMSceneMgrMessageType msg_type):
    std::runtime_error(std::format("Unknown scene manager message type: {}", msg_type)) { }
};

// Objects:

// Scenes:


}

#endif // FILE_GM_EXCEPTIONS_HPP_INCLUDED
