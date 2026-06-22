/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file contains the tests for the math functions and classes.

    Run only math tests:
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

    v1 = {15.0f32, 20.0f32};
    v1.gm_div(10.0f32);

    REQUIRE(gm_approx(v1.x, 1.5f32) && gm_approx(v1.y, 2.0f32));

    v1 = {5.5f32, 3.3f32};
    v2 = v1.gm_norm1();
    REQUIRE(gm_approx(v1.x, 5.5f32) && gm_approx(v1.y, 3.3f32));
    REQUIRE(gm_approx(v2.x, 0.8574929f32) && gm_approx(v2.y, 0.5144958f32));
    REQUIRE(gm_approx(v2.gm_len1(), 1.0f32));

    v3.gm_norm2();
    REQUIRE(gm_approx(v3.x, 0.3671665f32) && gm_approx(v3.y, 0.9301552f32));
    REQUIRE(gm_approx(v3.gm_len1(), 1.0f32));

    v1 = {50.0f32, 20.0f32};
    v2 = {80.0f32, 60.0f32};
    REQUIRE(gm_approx(v1.gm_dist1(v2), 50.0f32));
    REQUIRE(gm_approx(v1.gm_dist2(v2), 2500.0f32));
    REQUIRE(gm_approx(v1.gm_cross(v2), 1400.0f32));
    REQUIRE(gm_approx(v1.gm_cross(v1), 0.0f32));

    v1 = {10.0f32, 0.0f32};
    REQUIRE(gm_approx(v1.gm_angle(), 0.0f32));
    v1 = {10.0f32, 10.0f32};
    REQUIRE(gm_approx(v1.gm_angle(), 45.0f32));
    v1 = {0.0f32, 10.0f32};
    REQUIRE(gm_approx(v1.gm_angle(), 90.0f32));
    v1 = {-10.0f32, 10.0f32};
    REQUIRE(gm_approx(v1.gm_angle(), 135.0f32));

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

TEST_CASE("Create and use GMLine", "[math]" ) {
    GMLine l1{10.0f32, 5.0f32, 40.0f32, 45.0f32};
    GMLine l2{10.0f32, 10.0f32, 60.0f32, 60.0f32};

    REQUIRE(l1.gm_len1() == 50.0f32);
    REQUIRE(l1.gm_len2() == 2500.0f32);

    REQUIRE(l2.gm_len1() == 70.71068f32);

    l2.gm_scale(0.1414213562373095f32);
    REQUIRE(l2.gm_len1() == 10.0f32);
    REQUIRE(l2.gm_angle() == 45.00f32);

    l2.gm_rotate(45.0f32);

    REQUIRE((l2.v1.x == 10.0f32) && (l2.v1.y == 10.0f32));
    REQUIRE((l2.v2.x == 10.0f32) && (l2.v2.y == 20.0f32));
    REQUIRE(l2.gm_angle() == 90.00f32);

    GMLine l3{10.0f32, 10.0f32, 10.0f32, 20.0f32};
    REQUIRE(l1 != l2);
    REQUIRE(l2 == l3);
}

TEST_CASE("Create and use GMCircle", "[math]" ) {
    GMCircle c1{10.0f32, 10.0f32, 5.0f32};
    GMCircle c2{10.0f32, 10.0f32, 15.0f32};

    REQUIRE(c1 != c2);
    c1.gm_scale(3.0f32);
    REQUIRE(c1 == c2);
}

TEST_CASE("Create and use GMRectangle", "[math]" ) {
    GMRectangle r1{10.0f32, 10.0f32, 50.0f32, 40.0f32};
    GMRectangle r2{20.0f32, 30.0f32, 60.0f32, 150.0f32};

    REQUIRE(r1.gm_width() == 40.0f32);
    REQUIRE(r1.gm_height() == 30.0f32);
    REQUIRE(r1.gm_diagonal1() == 50.0f32);
    REQUIRE(r1.gm_diagonal2() == 2500.0f32);
    REQUIRE(r1.gm_min_point() == GMVec2D(10.0f32, 10.0f32));
    REQUIRE(r1.gm_max_point() == GMVec2D(50.0f32, 40.0f32));
    r1.gm_scale(2.0f32);
    REQUIRE(r1.gm_width() == 80.0f32);
    REQUIRE(r1.gm_height() == 60.0f32);
    REQUIRE(r1.gm_diagonal1() == 100.0f32);
    REQUIRE((r1.v1.x == 10.0f32) && (r1.v1.y == 10.0f32));
    REQUIRE((r1.v2.x == 90.0f32) && (r1.v2.y == 70.0f32));

    GMRectangle r3{10.0f32, 10.0f32, 90.0f32, 70.0f32};
    REQUIRE(r1 != r2);
    REQUIRE(r1 == r3);
}

TEST_CASE("Test helper functions", "[math]" ) {
    REQUIRE(gm_approx(2.5f32, 2.5f32));
    REQUIRE(gm_approx(2.5f32, 2.500000001f32));
    REQUIRE(!gm_approx(2.5f32, 2.51f32));

    REQUIRE(gm_approx(GMVec2D(1.5f32, 3.2f32), GMVec2D(1.5f32, 3.2f32)));
    REQUIRE(gm_approx(GMVec2D(1.5f32, 3.2f32), GMVec2D(1.5f32, 3.20000001f32)));
    REQUIRE(!gm_approx(GMVec2D(1.5f32, 3.2f32), GMVec2D(1.5f32, 3.21f32)));

    REQUIRE(gm_is_on_segment(GMVec2D(2.0f32, 4.0f32), GMVec2D(1.0f32, 4.0f32), GMVec2D(7.0f32, 4.0f32)));
    REQUIRE(!gm_is_on_segment(GMVec2D(0.5f32, 4.0f32), GMVec2D(1.0f32, 4.0f32), GMVec2D(7.0f32, 4.0f32)));
    REQUIRE(!gm_is_on_segment(GMVec2D(8.0f32, 4.0f32), GMVec2D(1.0f32, 4.0f32), GMVec2D(7.0f32, 4.0f32)));
    REQUIRE(!gm_is_on_segment(GMVec2D(2.0f32, 3.0f32), GMVec2D(1.0f32, 4.0f32), GMVec2D(7.0f32, 4.0f32)));
    REQUIRE(gm_is_on_segment(GMVec2D(2.0f32, 2.0f32), GMVec2D(1.0f32, 1.0f32), GMVec2D(7.0f32, 7.0f32)));

    // gm_orientation

    // gm_between

    // gm_closest_point_on_segment
}

TEST_CASE("Test intersect functions", "[math]" ) {
    // TODO: implement test cases.
}
