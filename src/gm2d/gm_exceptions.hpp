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

namespace gm2d {

class GMUnknownMessageType: public std::runtime_error {
public:
  GMUnknownMessageType(GMMessageType msg_type, std::string_view obj_name):
    std::runtime_error(std::format("Unknown message type: {}, object name: {}", msg_type, obj_name)) { }
};

class GMObjectNameDuplicate: public std::runtime_error {
public:
  GMObjectNameDuplicate(std::string_view where, std::string_view obj_name):
    std::runtime_error(std::format("Object already exists ({}): {}", where, obj_name)) { }
};

class GMObjectNotFound: public std::runtime_error {
public:
  GMObjectNotFound(std::string_view where, std::string_view obj_name):
    std::runtime_error(std::format("Object not found ({}): {}", where, obj_name)) { }
};

}

#endif // FILE_GM_EXCEPTIONS_HPP_INCLUDED
