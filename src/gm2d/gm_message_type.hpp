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

namespace gm2d {
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
}

template <>
struct std::formatter<gm2d::GMMessageType> : std::formatter<std::string_view> {
    auto format(gm2d::GMMessageType m, format_context& ctx) const {
        std::string_view name;

        switch (m) {
            case gm2d::GMMessageType::SetActive:
                name = "SetActive";
            break;

            case gm2d::GMMessageType::GetActive:
                name = "GetActive";
            break;

            case gm2d::GMMessageType::ToggleActive:
                name = "ToggleActive";
            break;

            case gm2d::GMMessageType::SetUpdateOrder:
                name = "SetUpdateOrder";
            break;

            case gm2d::GMMessageType::GetUpdateOrder:
                name = "GetUpdateOrder";
            break;

            case gm2d::GMMessageType::SetVisible:
                name = "SetVisible";
            break;

            case gm2d::GMMessageType::GetVisible:
                name = "GetVisible";
            break;

            case gm2d::GMMessageType::ToggleVisible:
                name = "ToggleVisible";
            break;

            case gm2d::GMMessageType::SetDrawOrder:
                name = "SetDrawOrder";
            break;

            case gm2d::GMMessageType::GetDrawOrder:
                name = "GetDrawOrder";
            break;

            case gm2d::GMMessageType::Custom:
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
