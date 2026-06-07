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
class GMUnknownMessageType: public std::runtime_error {
public:
  GMUnknownMessageType(GMMessageType msg_type, GMStringId obj_name):
    std::runtime_error(std::format("Unknown message type: {}, object id: {}", msg_type, obj_name.value)) { }
};

class GMObjectNameDuplicate: public std::runtime_error {
public:
  GMObjectNameDuplicate(std::string_view where, GMStringId obj_name):
    std::runtime_error(std::format("Object already exists ({}): {}", where, obj_name.value)) { }
};

class GMObjectNotFound: public std::runtime_error {
public:
  GMObjectNotFound(std::string_view where, GMStringId obj_name):
    std::runtime_error(std::format("Object not found ({}): {}", where, obj_name.value)) { }
};

class GMInvalidReceiver: public std::runtime_error {
public:
  GMInvalidReceiver(std::string_view where, GMStringId receiver):
    std::runtime_error(std::format("Invalid receiver ({}): {}", where, receiver.value)) { }
};

/*
class GMInvalidCategory: public std::runtime_error {
public:
  GMInvalidCategory(std::string_view where, GMMessageCategory category, std::string_view data):
    std::runtime_error(std::format("Invalid category ({}): category: {}, data: {}", where, category, data)) { }
  GMInvalidCategory(std::string_view where, GMMessageCategory category):
    std::runtime_error(std::format("Invalid category ({}): category: {}", where, category)) { }
};
*/

}

#endif // FILE_GM_EXCEPTIONS_HPP_INCLUDED
