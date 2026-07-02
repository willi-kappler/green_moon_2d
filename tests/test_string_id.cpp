/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file contains the tests for the string id class.

    Run only configuration tests:
    xmake run -w ./ gm2d_test [string_id]
*/

// External includes:
#include <snitch/snitch.hpp>

// Local includes:
#include "gm2d/gm_string_id.hpp"

using namespace gm2d;

TEST_CASE("Create and use GMStringId", "[string_id]" ) {
    REQUIRE(GMID("") == GMID(""));
    REQUIRE(GMID("A") == GMID("A"));
    REQUIRE(GMID("A") != GMID("AB"));
    REQUIRE(GMID("Foo") == GMID("Foo"));
    REQUIRE(GMID("Foo") != GMID("FOO"));
    REQUIRE(GMID("Foo") != GMID("Bar"));
}
