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
enum struct GMMessageSenderType: uint8_t {
    Object = 0,
    ObjectManager,
    Scene,
    SceneManager
};

enum struct GMObjectMessageType: uint16_t {
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

    // BitmapText:
    SetText,
    GetText,
    GetTextResult,
    SetFont,
    SetHorizontal,
    SetSineEffect,
    SetShakeEffect,
    SetRotationEffect,

    // Special messages:
    MessageToChild,
    MultiMessage,

    // Custom messages:
    Custom,
    CustomResult
};

enum struct GMObjMgrMessageType: uint8_t {
    // Object manager messages:
    AddLayer = 0,
    RemoveLayer,
    ClearLayer,
    ClearAllLayers,
    AddObject,
    RemoveObject,
    ReplaceObject,
    ClearAllObjects,
    MoveToLayer1,
    MoveToLayer2
};

enum struct GMSceneMessageType: uint8_t {
    // Scene messages:
    // Custom messages:
    Custom = 0,
    CustomResult
};

enum struct GMSceneMgrMessageType: uint8_t {
    // Scene manager messages:
    AddScene = 0,
    RemoveScene,
    ReplaceScene,
    ChangeToScene,
    PushAndChange,
    PopAndChange,
    UpdateStackTop,
    DrawStackTop,
    UpdateScene,
    DrawScene
};

enum struct GMEngineMessageType: uint8_t {
    Quit = 0,
    SetFullscreen,
    ToggleFullscreen,
    IncreaseSoundVolume,
    DecreaseSoundVolume,
    IncreaseMusicVolume,
    DecreaseMusicVolume
};
}

template <>
struct std::formatter<gm2d::GMMessageSenderType> : std::formatter<std::string_view> {
    auto format(gm2d::GMMessageSenderType m, format_context& ctx) const {
        std::string name;

        switch (m) {
            case gm2d::GMMessageSenderType::Object:
                name = "Object";
            break;

            case gm2d::GMMessageSenderType::ObjectManager:
                name = "ObjectManager";
            break;

            case gm2d::GMMessageSenderType::Scene:
                name = "Scene";
            break;

            case gm2d::GMMessageSenderType::SceneManager:
                name = "SceneManager";
            break;

            default:
                name = std::format("Unknown scene manager message type: {}", static_cast<uint8_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

template <>
struct std::formatter<gm2d::GMObjectMessageType> : std::formatter<std::string_view> {
    auto format(gm2d::GMObjectMessageType m, format_context& ctx) const {
        std::string name;

        switch (m) {
            case gm2d::GMObjectMessageType::SetActive:
                name = "SetActive";
            break;

            case gm2d::GMObjectMessageType::GetActive:
                name = "GetActive";
            break;

            case gm2d::GMObjectMessageType::GetActiveResult:
                name = "GetActiveResult";
            break;

            case gm2d::GMObjectMessageType::ToggleActive:
                name = "ToggleActive";
            break;

            case gm2d::GMObjectMessageType::SetUpdateOrder:
                name = "SetUpdateOrder";
            break;

            case gm2d::GMObjectMessageType::GetUpdateOrder:
                name = "GetUpdateOrder";
            break;

            case gm2d::GMObjectMessageType::GetUpdateOrderResult:
                name = "GetUpdateOrderResult";
            break;

            case gm2d::GMObjectMessageType::AddGroup:
                name = "AddGroup";
            break;

            case gm2d::GMObjectMessageType::RemoveGroup:
                name = "RemoveGroup";
            break;

            case gm2d::GMObjectMessageType::ClearGroups:
                name = "ClearGroups";
            break;

            case gm2d::GMObjectMessageType::SetVisible:
                name = "SetVisible";
            break;

            case gm2d::GMObjectMessageType::GetVisible:
                name = "GetVisible";
            break;

            case gm2d::GMObjectMessageType::GetVisibleResult:
                name = "GetVisibleResult";
            break;

            case gm2d::GMObjectMessageType::ToggleVisible:
                name = "ToggleVisible";
            break;

            case gm2d::GMObjectMessageType::SetDrawOrder:
                name = "SetDrawOrder";
            break;

            case gm2d::GMObjectMessageType::GetDrawOrder:
                name = "GetDrawOrder";
            break;

            case gm2d::GMObjectMessageType::GetDrawOrderResult:
                name = "GetDrawOrderResult";
            break;

            case gm2d::GMObjectMessageType::SetPosition:
                name = "SetPosition";
            break;

            case gm2d::GMObjectMessageType::GetPosition:
                name = "GetPosition";
            break;

            case gm2d::GMObjectMessageType::GetPositionResult:
                name = "GetPositionResult";
            break;

            case gm2d::GMObjectMessageType::AddPosition:
                name = "AddPosition";
            break;

            case gm2d::GMObjectMessageType::MessageToChild:
                name = "MessageToChild";
            break;

            case gm2d::GMObjectMessageType::MultiMessage:
                name = "MultiMessage";
            break;

            case gm2d::GMObjectMessageType::Custom:
                name = "Custom";
            break;

            case gm2d::GMObjectMessageType::CustomResult:
                name = "CustomResult";
            break;

            default:
                name = std::format("Unknown object message type: {}", static_cast<uint16_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

template <>
struct std::formatter<gm2d::GMObjMgrMessageType> : std::formatter<std::string_view> {
    auto format(gm2d::GMObjMgrMessageType m, format_context& ctx) const {
        std::string name;

        switch (m) {
            case gm2d::GMObjMgrMessageType::AddLayer:
                name = "AddLayer";
            break;

            case gm2d::GMObjMgrMessageType::RemoveLayer:
                name = "RemoveLayer";
            break;

            case gm2d::GMObjMgrMessageType::ClearLayer:
                name = "ClearLayer";
            break;

            case gm2d::GMObjMgrMessageType::ClearAllLayers:
                name = "ClearAllLayers";
            break;

            case gm2d::GMObjMgrMessageType::AddObject:
                name = "AddObject";
            break;

            case gm2d::GMObjMgrMessageType::RemoveObject:
                name = "RemoveObject";
            break;

            case gm2d::GMObjMgrMessageType::ReplaceObject:
                name = "ReplaceObject";
            break;

            case gm2d::GMObjMgrMessageType::ClearAllObjects:
                name = "ClearAllObjects";
            break;

            case gm2d::GMObjMgrMessageType::MoveToLayer1:
                name = "MoveToLayer1";
            break;

            case gm2d::GMObjMgrMessageType::MoveToLayer2:
                name = "MoveToLayer2";
            break;

            default:
                name = std::format("Unknown object manager message type: {}", static_cast<uint8_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

template <>
struct std::formatter<gm2d::GMSceneMessageType> : std::formatter<std::string_view> {
    auto format(gm2d::GMSceneMessageType m, format_context& ctx) const {
        std::string name;

        switch (m) {
            case gm2d::GMSceneMessageType::Custom:
                name = "Custom";
            break;

            case gm2d::GMSceneMessageType::CustomResult:
                name = "CustomResult";
            break;

            default:
                name = std::format("Unknown scene message type: {}", static_cast<uint8_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

template <>
struct std::formatter<gm2d::GMSceneMgrMessageType> : std::formatter<std::string_view> {
    auto format(gm2d::GMSceneMgrMessageType m, format_context& ctx) const {
        std::string name;

        switch (m) {
            case gm2d::GMSceneMgrMessageType::AddScene:
                name = "AddScene";
            break;

            case gm2d::GMSceneMgrMessageType::RemoveScene:
                name = "RemoveScene";
            break;

            case gm2d::GMSceneMgrMessageType::ReplaceScene:
                name = "ReplaceScene";
            break;

            case gm2d::GMSceneMgrMessageType::ChangeToScene:
                name = "ChangeToScene";
            break;

            case gm2d::GMSceneMgrMessageType::PushAndChange:
                name = "PushAndChange";
            break;

            case gm2d::GMSceneMgrMessageType::PopAndChange:
                name = "PopAndChange";
            break;

            case gm2d::GMSceneMgrMessageType::UpdateStackTop:
                name = "UpdateStackTop";
            break;

            case gm2d::GMSceneMgrMessageType::DrawStackTop:
                name = "DrawStackTop";
            break;

            case gm2d::GMSceneMgrMessageType::UpdateScene:
                name = "UpdateScene";
            break;

            case gm2d::GMSceneMgrMessageType::DrawScene:
                name = "DrawScene";
            break;

            default:
                name = std::format("Unknown scene manager message type: {}", static_cast<uint8_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

template <>
struct std::formatter<gm2d::GMEngineMessageType> : std::formatter<std::string_view> {
    auto format(gm2d::GMEngineMessageType m, format_context& ctx) const {
        std::string name;

        switch (m) {
            case gm2d::GMEngineMessageType::Quit:
                name = "Quit";
            break;

            case gm2d::GMEngineMessageType::SetFullscreen:
                name = "SetFullscreen";
            break;

            case gm2d::GMEngineMessageType::ToggleFullscreen:
                name = "ToggleFullscreen";
            break;

            case gm2d::GMEngineMessageType::IncreaseSoundVolume:
                name = "IncreaseSoundVolume";
            break;

            case gm2d::GMEngineMessageType::DecreaseSoundVolume:
                name = "DecreaseSoundVolume";
            break;

            case gm2d::GMEngineMessageType::IncreaseMusicVolume:
                name = "IncreaseMusicVolume";
            break;

            case gm2d::GMEngineMessageType::DecreaseMusicVolume:
                name = "DecreaseMusicVolume";
            break;

            default:
                name = std::format("Unknown scene manager message type: {}", static_cast<uint8_t>(m));
            break;
        }

        return std::formatter<std::string_view>::format(name, ctx);
    }
};

#endif // FILE_GM_MESSAGE_TYPE_HPP_INCLUDED
