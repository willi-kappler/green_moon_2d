/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the configuration options
*/

// STD includes:
#include <fstream>
#include <iostream>
#include <print>

// Local includes:
#include "gm_configuration.hpp"
#include "gm_exceptions.hpp"

namespace gm2d {
GMConfiguration::GMConfiguration():
    config_file("config.json"),
    fps(60),
    fullscreen(false),
    resource_file("resources.json"),
    screen_width(800),
    screen_height(600),
    window_title("Made with GreenMoon2D")
{}

[[nodiscard]] GMConfiguration gm_config_from_json(const tao::json::value json_config) {
    GMConfiguration gm_config;

    if (auto v = json_config.find("config_file"); v != nullptr) {
        gm_config.config_file = v->as<std::string>();

        if (gm_config.config_file.size() == 0) {
            throw GMConfigurationException("config_file is empty!");
        }
    }

    if (auto v = json_config.find("fps"); v != nullptr) {
        gm_config.fps = v->as<uint8_t>();

        if (gm_config.fps < 10) {
            throw GMConfigurationException("fps < 10!");
        }
    }

    if (auto v = json_config.find("fullscreen"); v != nullptr) {
        gm_config.fullscreen = v->as<bool>();
    }

    if (auto v = json_config.find("resource_file"); v != nullptr) {
        gm_config.resource_file = v->as<std::string>();

        if (gm_config.resource_file.size() == 0) {
            throw GMConfigurationException("resource_file is empty!");
        }
    }

    if (auto v = json_config.find("screen_width"); v != nullptr) {
        gm_config.screen_width = v->as<uint16_t>();

        if (gm_config.screen_width < 320) {
            throw GMConfigurationException("screen_width < 320!");
        }
    }

    if (auto v = json_config.find("screen_height"); v != nullptr) {
        gm_config.screen_height = v->as<uint16_t>();

        if (gm_config.screen_height < 240) {
            throw GMConfigurationException("screen_height < 240!");
        }
    }

    if (auto v = json_config.find("window_title"); v != nullptr) {
        gm_config.window_title = v->as<std::string>();

        if (gm_config.window_title.size() == 0) {
            throw GMConfigurationException("window_title is empty!");
        }
    }

    return gm_config;
}

[[nodiscard]] std::string gm_file_to_string(std::filesystem::path file_path) {
    std::ifstream in_file(file_path);

    if (in_file.is_open()) {
        std::string file_contents {std::istreambuf_iterator<char>(in_file), std::istreambuf_iterator<char>()};
        return file_contents;
    } else {
        throw GMConfigurationException("Open file error, read config");
    }
}

[[nodiscard]] GMConfiguration gm_config_from_string(std::string_view config_as_string) {
    const tao::json::value json_config = tao::json::from_string(config_as_string);

    return gm_config_from_json(json_config);
}

[[nodiscard]] GMConfiguration gm_config_from_file(std::filesystem::path file_path) {
    return gm_config_from_string(gm_file_to_string(file_path));
}

void gm_save_config(GMConfiguration gm_config) {
    gm_save_config(gm_config, gm_config.config_file);
}

void gm_save_config(GMConfiguration gm_config, std::filesystem::path file_path) {
    // std::cout << gm_config.config_file << std::endl;
    // std::cout << file_path << std::endl;

    const tao::json::value json_data = {
        {"config_file", gm_config.config_file},
        {"fps", gm_config.fps},
        {"fullscreen", gm_config.fullscreen},
        {"resource_file", gm_config.resource_file},
        {"screen_width", gm_config.screen_width},
        {"screen_height", gm_config.screen_height},
        {"window_title", gm_config.window_title}
    };

    const std::string serialized = tao::json::to_string(json_data);

    std::ofstream out_file(file_path);

    if (out_file.is_open()) {
        std::print(out_file, "{}", serialized);
    } else {
        throw GMConfigurationException("Open file error, write config");
    }
}
}
