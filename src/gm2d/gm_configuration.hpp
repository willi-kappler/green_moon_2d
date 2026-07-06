/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the configuration options
*/

#ifndef FILE_GM_CONFIG_HPP_INCLUDED
#define FILE_GM_CONFIG_HPP_INCLUDED

// STD includes:
#include <string>
#include <cstdint>
#include <string_view>
#include <filesystem>

// External includes:
#include <tao/json.hpp>

namespace gm2d {
class GMConfiguration {
    public:
        // Constructor:
        GMConfiguration();

        // Members:
        std::string config_file;
        uint8_t fps;
        bool fullscreen;
        std::string resource_file;
        uint16_t screen_width;
        uint16_t screen_height;
        std::string window_title;

        // TODO:
        // audio_volume: uint8_t, music_volume: uint8_t, difficulty: uint8_t
        // input_config_file: string
};

[[nodiscard]] GMConfiguration gm_config_from_json(const tao::json::value);

[[nodiscard]] std::string gm_file_to_string(std::filesystem::path);

[[nodiscard]] GMConfiguration gm_config_from_string(std::string_view);

[[nodiscard]] GMConfiguration gm_config_from_file(std::filesystem::path);

void gm_save_config(GMConfiguration);

void gm_save_config(GMConfiguration, std::filesystem::path);
}

#endif // FILE_GM_CONFIG_HPP_INCLUDED
