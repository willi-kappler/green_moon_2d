/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file contains the tests for the math functions and classes.

    Run only configuration tests:
    xmake run -w ./ gm2d_test [math]
*/

// External includes:
#include <snitch/snitch.hpp>

// Local includes:
#include "gm2d/gm_math.hpp"

using namespace gm2d;

TEST_CASE("Create and use GMVec2D", "[math]" ) {
    GMVec2D v1, v2(-5.5f32), v3(1.5f32, 3.8f32);
    REQUIRE((v1.x == 0.0) && (v1.y == 0.0));
    REQUIRE((v2.x == -5.5f32) && (v2.y == -5.5f32));
    REQUIRE((v3.x == 1.5f32) && (v3.y == 3.8f32));

    GMVec2D v4(v3);
    REQUIRE((v3.x == 1.5f32) && (v3.y == 3.8f32));
    REQUIRE((v4.x == 1.5f32) && (v4.y == 3.8f32));

    v1.gm_add(1.2f32);
    REQUIRE((v1.x == 1.2f32) && (v1.y == 1.2f32));

    v1.gm_add({1.8f32, 2.8f32});
    REQUIRE((v1.x == 3.0f32) && (v1.y == 4.0f32));

    v1.gm_add(v2);
    REQUIRE((v1.x == -2.5f32) && (v1.y == -1.5f32));

    v1.gm_mul(-2.2f32);
    REQUIRE(gm_approx(v1.x, 5.5f32) && gm_approx(v1.y, 3.3f32));

    REQUIRE(gm_approx(v1.gm_len1(), 6.414047f32));
    REQUIRE(gm_approx(v1.gm_len2(), 41.14000f32));

    v2 = v1.gm_norm1();
    REQUIRE(gm_approx(v1.x, 5.5f32) && gm_approx(v1.y, 3.3f32));
    REQUIRE(gm_approx(v2.x, 0.8574929f32) && gm_approx(v2.y, 0.5144958f32));
    REQUIRE(gm_approx(v2.gm_len1(), 1.0f32));

    v3.gm_norm2();
    REQUIRE(gm_approx(v3.x, 0.3671665f32) && gm_approx(v3.y, 0.9301552f32));
    REQUIRE(gm_approx(v3.gm_len1(), 1.0f32));

    v1 = {5.0f32, 0.0f32};
    v2 = v1.gm_rotate1(90.0f32);
    REQUIRE(gm_approx(v1.x, 5.0f32) && gm_approx(v1.y, 0.0f32));
    REQUIRE(gm_approx(v2.x, 0.0f32) && gm_approx(v2.y, 5.0f32));

    v3 = v1;
    v3.gm_rotate2(90.0f32);
    REQUIRE(gm_approx(v1.x, 5.0f32) && gm_approx(v1.y, 0.0f32));
    REQUIRE(gm_approx(v3.x, 0.0f32) && gm_approx(v3.y, 5.0f32));

    v1 = {2.5f32, 5.0f32};
    v2 = {2.5f32, 5.0f32};
    v3 = {-1.2f32, 7.8f32};
    REQUIRE(v1 == v2);
    REQUIRE(v1 != v3);

    v2 = v1 + v3;
    REQUIRE(gm_approx(v1.x, 2.5f32) && gm_approx(v1.y, 5.0f32));
    REQUIRE(gm_approx(v2.x, 1.3f32) && gm_approx(v2.y, 12.8f32));
    REQUIRE(gm_approx(v3.x, -1.2f32) && gm_approx(v3.y, 7.8f32));

    v2 = v1 + 10.0f32;
    REQUIRE(gm_approx(v1.x, 2.5f32) && gm_approx(v1.y, 5.0f32));
    REQUIRE(gm_approx(v2.x, 12.5f32) && gm_approx(v2.y, 15.0f32));

    v1 += v3;
    REQUIRE(gm_approx(v1.x, 1.3f32) && gm_approx(v1.y, 12.8f32));
    REQUIRE(gm_approx(v3.x, -1.2f32) && gm_approx(v3.y, 7.8f32));

    v1 += 5.0f32;
    REQUIRE(gm_approx(v1.x, 6.3f32) && gm_approx(v1.y, 17.8f32));

    v1 = {2.5f32, 5.0f32};
    v2 = {1.5f32, 3.0f32};
    v3 = v1 - v2;
    REQUIRE(gm_approx(v1.x, 2.5f32) && gm_approx(v1.y, 5.0f32));
    REQUIRE(gm_approx(v2.x, 1.5f32) && gm_approx(v2.y, 3.0f32));
    REQUIRE(gm_approx(v3.x, 1.0f32) && gm_approx(v3.y, 2.0f32));

    v3 = v1 - 1.0f32;
    REQUIRE(gm_approx(v1.x, 2.5f32) && gm_approx(v1.y, 5.0f32));
    REQUIRE(gm_approx(v3.x, 1.5f32) && gm_approx(v3.y, 4.0f32));

    v1 -= v2;
    REQUIRE(gm_approx(v1.x, 1.0f32) && gm_approx(v1.y, 2.0f32));
    REQUIRE(gm_approx(v2.x, 1.5f32) && gm_approx(v2.y, 3.0f32));

    v1 -= 0.5f32;
    REQUIRE(gm_approx(v1.x, 0.5f32) && gm_approx(v1.y, 1.5f32));

    v2 = v1 * 10.0f32;
    REQUIRE(gm_approx(v1.x, 0.5f32) && gm_approx(v1.y, 1.5f32));
    REQUIRE(gm_approx(v2.x, 5.0f32) && gm_approx(v2.y, 15.0f32));

    v1 *= 5.0f32;
    REQUIRE(gm_approx(v1.x, 2.5f32) && gm_approx(v1.y, 7.5f32));

    // INFO("v3x: ", v3.x, ", v3y: ", v3.y);


}
