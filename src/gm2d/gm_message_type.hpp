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
    Empty = 0,

    // Normal objects:
    SetActive,
    GetActive,
    GetActiveResult,
    ToggleActive,

    SetUpdateOrder,
    GetUpdateOrder,
    GetUpdateOrderResult,

    AddGroup,
    RemoveGroup,
    ClearGroups,

    // GFX objects:
    SetVisible,
    GetVisible,
    GetVisibleResult,
    ToggleVisible,

    SetDrawOrder,
    GetDrawOrder,
    GetDrawOrderResult,

    SetPosition,
    GetPosition,
    GetPositionResult,
    AddPosition,

    // Object manager messages:
    AddObject,
    RemoveObject,
    ReplaceObject,
    ClearObjects,

    // Scene manager messages:
    AddScene,
    RemoveScene,
    ChangeToScene,

    // Special messages:
    MessageToChild,
    MultiMessage,

    // Custom messages:
    Custom,
    CustomResult
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

            case gm2d::GMMessageType::GetActiveResult:
                name = "GetActiveResult";
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

            case gm2d::GMMessageType::GetUpdateOrderResult:
                name = "GetUpdateOrderResult";
            break;

            case gm2d::GMMessageType::AddGroup:
                name = "AddGroup";
            break;

            case gm2d::GMMessageType::RemoveGroup:
                name = "RemoveGroup";
            break;

            case gm2d::GMMessageType::ClearGroups:
                name = "ClearGroups";
            break;

            case gm2d::GMMessageType::SetVisible:
                name = "SetVisible";
            break;

            case gm2d::GMMessageType::GetVisible:
                name = "GetVisible";
            break;

            case gm2d::GMMessageType::GetVisibleResult:
                name = "GetVisibleResult";
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

            case gm2d::GMMessageType::GetDrawOrderResult:
                name = "GetDrawOrderResult";
            break;

            case gm2d::GMMessageType::SetPosition:
                name = "SetPosition";
            break;

            case gm2d::GMMessageType::GetPosition:
                name = "GetPosition";
            break;

            case gm2d::GMMessageType::GetPositionResult:
                name = "GetPositionResult";
            break;

            case gm2d::GMMessageType::AddPosition:
                name = "AddPosition";
            break;

            case gm2d::GMMessageType::MessageToChild:
                name = "MessageToChild";
            break;

            case gm2d::GMMessageType::MultiMessage:
                name = "MultiMessage";
            break;

            case gm2d::GMMessageType::Custom:
                name = "Custom";
            break;

            case gm2d::GMMessageType::CustomResult:
                name = "CustomResult";
            break;

            default:
                name = std::format("Unknown message type: {}", static_cast<uint16_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

#endif // FILE_GM_MESSAGE_TYPE_HPP_INCLUDED
