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
enum struct GMNormalMessageType: uint8_t {
    // Normal objects:
    SetActive = 0,
    GetActive,
    GetActiveResult,
    ToggleActive,

    SetUpdateOrder,
    GetUpdateOrder,
    GetUpdateOrderResult,

    AddGroup,
    RemoveGroup,
    ClearGroups,

    // Custom messages:
    Custom,
    CustomResult
};

enum struct GMGFXMessageType: uint8_t {
    // GFX objects:
    SetVisible = 0,
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

    // Custom messages:
    Custom,
    CustomResult
};

enum struct GMObjMgrMessageType: uint8_t {
    // Object manager messages:
    AddObject = 0,
    RemoveObject,
    AddObjectToGroup,
    RemoveObjectFromGroup,

    // Custom messages:
    Custom,
    CustomResult
};

enum struct GMSceneMgrMessageType: uint8_t {
    // Scene manager messages:
    AddScene = 0,
    RemoveScene,
    ChangeToScene,

    // Custom messages:
    Custom,
    CustomResult
};

/*
enum struct GMMessageType: uint8_t {
    // Special messages:
    MessageToChild,
    MultiMessage,

};
*/

enum struct GMMessageSenderKind {
    Normal = 0,
    GFX,
    ObjectManager,
    SceneManager,
};

}

template <>
struct std::formatter<gm2d::GMNormalMessageType> : std::formatter<std::string_view> {
    auto format(gm2d::GMNormalMessageType m, format_context& ctx) const {
        std::string_view name;

        switch (m) {
            case gm2d::GMNormalMessageType::SetActive:
                name = "SetActive";
            break;

            case gm2d::GMNormalMessageType::GetActive:
                name = "GetActive";
            break;

            case gm2d::GMNormalMessageType::GetActiveResult:
                name = "GetActiveResult";
            break;

            case gm2d::GMNormalMessageType::ToggleActive:
                name = "ToggleActive";
            break;

            case gm2d::GMNormalMessageType::SetUpdateOrder:
                name = "SetUpdateOrder";
            break;

            case gm2d::GMNormalMessageType::GetUpdateOrder:
                name = "GetUpdateOrder";
            break;

            case gm2d::GMNormalMessageType::GetUpdateOrderResult:
                name = "GetUpdateOrderResult";
            break;

            case gm2d::GMNormalMessageType::AddGroup:
                name = "AddGroup";
            break;

            case gm2d::GMNormalMessageType::RemoveGroup:
                name = "RemoveGroup";
            break;

            case gm2d::GMNormalMessageType::ClearGroups:
                name = "ClearGroups";
            break;

            case gm2d::GMNormalMessageType::Custom:
                name = "Custom";
            break;

            case gm2d::GMNormalMessageType::CustomResult:
                name = "CustomResult";
            break;

            default:
                name = std::format("Unknown normal message type: {}", static_cast<uint8_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

template <>
struct std::formatter<gm2d::GMGFXMessageType> : std::formatter<std::string_view> {
    auto format(gm2d::GMGFXMessageType m, format_context& ctx) const {
        std::string_view name;

        switch (m) {
            case gm2d::GMGFXMessageType::SetVisible:
                name = "SetVisible";
            break;

            case gm2d::GMGFXMessageType::GetVisible:
                name = "GetVisible";
            break;

            case gm2d::GMGFXMessageType::GetVisibleResult:
                name = "GetVisibleResult";
            break;

            case gm2d::GMGFXMessageType::ToggleVisible:
                name = "ToggleVisible";
            break;

            case gm2d::GMGFXMessageType::SetDrawOrder:
                name = "SetDrawOrder";
            break;

            case gm2d::GMGFXMessageType::GetDrawOrder:
                name = "GetDrawOrder";
            break;

            case gm2d::GMGFXMessageType::GetDrawOrderResult:
                name = "GetDrawOrderResult";
            break;

            case gm2d::GMGFXMessageType::SetPosition:
                name = "SetPosition";
            break;

            case gm2d::GMGFXMessageType::GetPosition:
                name = "GetPosition";
            break;

            case gm2d::GMGFXMessageType::GetPositionResult:
                name = "GetPositionResult";
            break;

            case gm2d::GMGFXMessageType::AddPosition:
                name = "AddPosition";
            break;

            case gm2d::GMGFXMessageType::Custom:
                name = "Custom";
            break;

            case gm2d::GMGFXMessageType::CustomResult:
                name = "CustomResult";
            break;

            default:
                name = std::format("Unknown gfx message type: {}", static_cast<uint8_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

template <>
struct std::formatter<gm2d::GMObjMgrMessageType> : std::formatter<std::string_view> {
    auto format(gm2d::GMObjMgrMessageType m, format_context& ctx) const {
        std::string_view name;

        switch (m) {
            case gm2d::GMObjMgrMessageType::AddObject:
                name = "AddObject";
            break;

            case gm2d::GMObjMgrMessageType::RemoveObject:
                name = "RemoveObject";
            break;

            case gm2d::GMObjMgrMessageType::AddObjectToGroup:
                name = "AddObjectToGroup";
            break;

            case gm2d::GMObjMgrMessageType::RemoveObjectFromGroup:
                name = "RemoveObjectFromGroup";
            break;

            case gm2d::GMObjMgrMessageType::Custom:
                name = "Custom";
            break;

            case gm2d::GMObjMgrMessageType::CustomResult:
                name = "CustomResult";
            break;

            default:
                name = std::format("Unknown object manager message type: {}", static_cast<uint8_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

template <>
struct std::formatter<gm2d::GMSceneMgrMessageType> : std::formatter<std::string_view> {
    auto format(gm2d::GMSceneMgrMessageType m, format_context& ctx) const {
        std::string_view name;

        switch (m) {
            case gm2d::GMSceneMgrMessageType::AddScene:
                name = "AddScene";
            break;

            case gm2d::GMSceneMgrMessageType::RemoveScene:
                name = "RemoveScene";
            break;

            case gm2d::GMSceneMgrMessageType::ChangeToScene:
                name = "ChangeToScene";
            break;

            case gm2d::GMSceneMgrMessageType::Custom:
                name = "Custom";
            break;

            case gm2d::GMSceneMgrMessageType::CustomResult:
                name = "CustomResult";
            break;

            default:
                name = std::format("Unknown scene manager message type: {}", static_cast<uint8_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

template <>
struct std::formatter<gm2d::GMMessageSenderKind> : std::formatter<std::string_view> {
    auto format(gm2d::GMMessageSenderKind m, format_context& ctx) const {
        std::string_view name;

        switch (m) {
            case gm2d::GMMessageSenderKind::Normal:
                name = "Normal";
            break;

            case gm2d::GMMessageSenderKind::GFX:
                name = "GFX";
            break;

            case gm2d::GMMessageSenderKind::ObjectManager:
                name = "ObjectManager";
            break;

            case gm2d::GMMessageSenderKind::SceneManager:
                name = "SceneManager";
            break;

            default:
                name = std::format("Unknown message category: {}", static_cast<uint16_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

#endif // FILE_GM_MESSAGE_TYPE_HPP_INCLUDED
