/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file contains the tests for the configuration class.

    Run only configuration tests:
    xmake run -w ./ gm2d_test [configuration]
*/

// External includes:
#include <snitch/snitch.hpp>

// Local includes:
#include "gm2d/gm_configuration.hpp"

using namespace gm2d;

TEST_CASE("Create default configuration", "[configuration]" ) {
    GMConfiguration config1;
    REQUIRE(config1.config_file == "config.json");
    REQUIRE(config1.fps == 60);
    REQUIRE(config1.fullscreen == false);
    REQUIRE(config1.resource_file == "resources.json");
    REQUIRE(config1.screen_width == 800);
    REQUIRE(config1.screen_height == 600);
    REQUIRE(config1.window_title == "Made with GreenMoon2D");
}

TEST_CASE("Create configuration from string", "[configuration]" ) {
    std::string config_string = R"json({
        "config_file": "foo_bar_config.json",
        "fps": 70,
        "fullscreen": true,
        "resource_file": "xxx_resource.json",
        "screen_width": 1024,
        "screen_height": 768,
        "window_title": "Test_Foo_Bar"
        })json";

    GMConfiguration config1 = gm_config_from_string(config_string);
    REQUIRE(config1.config_file == "config.json");
    REQUIRE(config1.fps == 70);
    REQUIRE(config1.fullscreen == true);
    REQUIRE(config1.resource_file == "xxx_resource.json");
    REQUIRE(config1.screen_width == 1024);
    REQUIRE(config1.screen_height == 768);
    REQUIRE(config1.window_title == "Test_Foo_Bar");
}
