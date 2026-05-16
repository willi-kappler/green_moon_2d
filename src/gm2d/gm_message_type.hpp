/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the message type and result type enums
*/

#ifndef FILE_GM_MESSAGE_TYPE_HPP_INCLUDED
#define FILE_GM_MESSAGE_TYPE_HPP_INCLUDED

// STD includes:
#include <cstdint>
#include <format>
#include <string_view>


enum struct GMMessageType: uint32_t {
    SetActive = 0,
    GetActive,
    ToggleActive,

    SetUpdateOrder,
    GetUpdateOrder,

    SetVisible,
    GetVisible,
    ToggleVisible,

    SetDrawOrder,
    GetDrawOrder,

    Custom
};

enum struct GMHandleResultType: uint32_t {
    Empty = 0,
    Active,
    UpdateOrder,
    Visible,
    DrawOrder,

    Custom
};


template <>
struct std::formatter<GMMessageType> : std::formatter<std::string_view> {
    auto format(GMMessageType m, format_context& ctx) const {
        std::string_view name;

        switch (m) {
            case GMMessageType::SetActive:
                name = "SetActive";
            break;

            case GMMessageType::GetActive:
                name = "GetActive";
            break;

            case GMMessageType::ToggleActive:
                name = "ToggleActive";
            break;

            case GMMessageType::SetUpdateOrder:
                name = "SetUpdateOrder";
            break;

            case GMMessageType::GetUpdateOrder:
                name = "GetUpdateOrder";
            break;

            case GMMessageType::SetVisible:
                name = "SetVisible";
            break;

            case GMMessageType::GetVisible:
                name = "GetVisible";
            break;

            case GMMessageType::ToggleVisible:
                name = "ToggleVisible";
            break;

            case GMMessageType::SetDrawOrder:
                name = "SetDrawOrder";
            break;

            case GMMessageType::GetDrawOrder:
                name = "GetDrawOrder";
            break;

            case GMMessageType::Custom:
                name = "Custom";
            break;

            default:
                name = std::format("Unknown: {}", static_cast<uint16_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

#endif // FILE_GM_MESSAGE_TYPE_HPP_INCLUDED
