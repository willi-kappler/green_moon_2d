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
    GMVec2D v1, v2(-5.5), v3(1.5, 3.8);
    REQUIRE((v1.x == 0.0f32) && (v1.y == 0.0f32));
    REQUIRE((v2.x == -5.5f32) && (v2.y == -5.5f32));
    REQUIRE((v3.x == 1.5f32) && (v3.y == 3.8f32));

    GMVec2D v4(v3);
    REQUIRE((v3.x == 1.5f32) && (v3.y == 3.8f32));
    REQUIRE((v4.x == 1.5f32) && (v4.y == 3.8f32));

    v1.gm_add(1.2);
    REQUIRE((v1.x == 1.2f32) && (v1.y == 1.2f32));

    v1.gm_add({1.8, 2.8});
    REQUIRE((v1.x == 3.0f32) && (v1.y == 4.0f32));

    v1.gm_add(v2);
    REQUIRE((v1.x == -2.5f32) && (v1.y == -1.5f32));

    v1.gm_mul(-2.2);
    REQUIRE(gm_approx(v1.x, 5.5) && gm_approx(v1.y, 3.3));

    REQUIRE(gm_approx(v1.gm_len1(), 6.414047));
    REQUIRE(gm_approx(v1.gm_len2(), 41.14000));

    v1 = {15.0, 20.0};
    v1.gm_div(10.0);

    REQUIRE(gm_approx(v1.x, 1.5) && gm_approx(v1.y, 2.0));

    v1 = {5.5, 3.3};
    v2 = v1.gm_norm1();
    REQUIRE(gm_approx(v1.x, 5.5) && gm_approx(v1.y, 3.3));
    REQUIRE(gm_approx(v2.x, 0.8574929) && gm_approx(v2.y, 0.5144958));
    REQUIRE(gm_approx(v2.gm_len1(), 1.0));

    v3.gm_norm2();
    REQUIRE(gm_approx(v3.x, 0.3671665) && gm_approx(v3.y, 0.9301552));
    REQUIRE(gm_approx(v3.gm_len1(), 1.0));

    v1 = {50.0, 20.0};
    v2 = {80.0, 60.0};
    REQUIRE(gm_approx(v1.gm_dist1(v2), 50.0));
    REQUIRE(gm_approx(v1.gm_dist2(v2), 2500.0));
    REQUIRE(gm_approx(v1.gm_cross(v2), 1400.0));
    REQUIRE(gm_approx(v1.gm_cross(v1), 0.0));

    v1 = {10.0, 0.0};
    REQUIRE(gm_approx(v1.gm_angle(), 0.0));
    v1 = {10.0, 10.0};
    REQUIRE(gm_approx(v1.gm_angle(), 45.0));
    v1 = {0.0, 10.0};
    REQUIRE(gm_approx(v1.gm_angle(), 90.0));
    v1 = {-10.0, 10.0};
    REQUIRE(gm_approx(v1.gm_angle(), 135.0));

    v1 = {5.0, 0.0};
    v2 = v1.gm_rotate1(90.0);
    REQUIRE(gm_approx(v1.x, 5.0) && gm_approx(v1.y, 0.0));
    REQUIRE(gm_approx(v2.x, 0.0) && gm_approx(v2.y, 5.0));

    v3 = v1;
    v3.gm_rotate2(90.0);
    REQUIRE(gm_approx(v1.x, 5.0) && gm_approx(v1.y, 0.0));
    REQUIRE(gm_approx(v3.x, 0.0) && gm_approx(v3.y, 5.0));

    v1 = {2.5, 5.0};
    v2 = {2.5, 5.0};
    v3 = {-1.2, 7.8};
    REQUIRE(v1 == v2);
    REQUIRE(v1 != v3);

    v2 = v1 + v3;
    REQUIRE(gm_approx(v1.x, 2.5) && gm_approx(v1.y, 5.0));
    REQUIRE(gm_approx(v2.x, 1.3) && gm_approx(v2.y, 12.8));
    REQUIRE(gm_approx(v3.x, -1.2) && gm_approx(v3.y, 7.8));

    v2 = v1 + 10.0;
    REQUIRE(gm_approx(v1.x, 2.5) && gm_approx(v1.y, 5.0));
    REQUIRE(gm_approx(v2.x, 12.5) && gm_approx(v2.y, 15.0));

    v1 += v3;
    REQUIRE(gm_approx(v1.x, 1.3) && gm_approx(v1.y, 12.8));
    REQUIRE(gm_approx(v3.x, -1.2) && gm_approx(v3.y, 7.8));

    v1 += 5.0;
    REQUIRE(gm_approx(v1.x, 6.3) && gm_approx(v1.y, 17.8));

    v1 = {2.5, 5.0};
    v2 = {1.5, 3.0};
    v3 = v1 - v2;
    REQUIRE(gm_approx(v1.x, 2.5) && gm_approx(v1.y, 5.0));
    REQUIRE(gm_approx(v2.x, 1.5) && gm_approx(v2.y, 3.0));
    REQUIRE(gm_approx(v3.x, 1.0) && gm_approx(v3.y, 2.0));

    v3 = v1 - 1.0;
    REQUIRE(gm_approx(v1.x, 2.5) && gm_approx(v1.y, 5.0));
    REQUIRE(gm_approx(v3.x, 1.5) && gm_approx(v3.y, 4.0));

    v1 -= v2;
    REQUIRE(gm_approx(v1.x, 1.0) && gm_approx(v1.y, 2.0));
    REQUIRE(gm_approx(v2.x, 1.5) && gm_approx(v2.y, 3.0));

    v1 -= 0.5;
    REQUIRE(gm_approx(v1.x, 0.5) && gm_approx(v1.y, 1.5));

    v2 = v1 * 10.0;
    REQUIRE(gm_approx(v1.x, 0.5) && gm_approx(v1.y, 1.5));
    REQUIRE(gm_approx(v2.x, 5.0) && gm_approx(v2.y, 15.0));

    v1 *= 5.0;
    REQUIRE(gm_approx(v1.x, 2.5) && gm_approx(v1.y, 7.5));

    // INFO("v3x: ", v3.x, ", v3y: ", v3.y);
}

TEST_CASE("Create and use GMLine", "[math]" ) {
    GMLine l1{10.0, 5.0, 40.0, 45.0};
    GMLine l2{10.0, 10.0, 60.0, 60.0};

    REQUIRE(l1.gm_len1() == 50.0);
    REQUIRE(l1.gm_len2() == 2500.0);

    REQUIRE(l2.gm_len1() == 70.71068f32);

    l2.gm_scale(0.1414213562373095);
    REQUIRE(l2.gm_len1() == 10.0);
    REQUIRE(l2.gm_angle() == 45.00);

    l2.gm_rotate(45.0);

    REQUIRE((l2.v1.x == 10.0) && (l2.v1.y == 10.0));
    REQUIRE((l2.v2.x == 10.0) && (l2.v2.y == 20.0));
    REQUIRE(l2.gm_angle() == 90.00);

    GMLine l3{10.0, 10.0, 10.0, 20.0};
    REQUIRE(l1 != l2);
    REQUIRE(l2 == l3);
}

TEST_CASE("Create and use GMCircle", "[math]" ) {
    GMCircle c1{10.0, 10.0, 5.0};
    GMCircle c2{10.0, 10.0, 15.0};

    REQUIRE(c1 != c2);
    c1.gm_scale(3.0);
    REQUIRE(c1 == c2);
}

TEST_CASE("Create and use GMRectangle", "[math]" ) {
    GMRectangle r1{10.0, 10.0, 50.0, 40.0};
    GMRectangle r2{20.0, 30.0, 60.0, 150.0};

    REQUIRE(r1.gm_width() == 40.0);
    REQUIRE(r1.gm_height() == 30.0);
    REQUIRE(r1.gm_diagonal1() == 50.0);
    REQUIRE(r1.gm_diagonal2() == 2500.0);
    REQUIRE(r1.gm_min_point() == GMVec2D(10.0, 10.0));
    REQUIRE(r1.gm_max_point() == GMVec2D(50.0, 40.0));
    r1.gm_scale(2.0);
    REQUIRE(r1.gm_width() == 80.0);
    REQUIRE(r1.gm_height() == 60.0);
    REQUIRE(r1.gm_diagonal1() == 100.0);
    REQUIRE((r1.v1.x == 10.0) && (r1.v1.y == 10.0));
    REQUIRE((r1.v2.x == 90.0) && (r1.v2.y == 70.0));

    GMRectangle r3{10.0, 10.0, 90.0, 70.0};
    REQUIRE(r1 != r2);
    REQUIRE(r1 == r3);
}

TEST_CASE("Test helper functions", "[math]" ) {
    REQUIRE(gm_approx(2.5, 2.5));
    REQUIRE(gm_approx(2.5, 2.500000001));
    REQUIRE(!gm_approx(2.5, 2.51));

    REQUIRE(gm_approx(3.9f32, 3.9f32));
    REQUIRE(gm_approx(3.9f32, 3.900000000001f32));
    REQUIRE(!gm_approx(3.9f32, 3.8f32));

    REQUIRE(gm_approx(GMVec2D(1.5, 3.2), GMVec2D(1.5, 3.2)));
    REQUIRE(gm_approx(GMVec2D(1.5, 3.2), GMVec2D(1.5, 3.20000001)));
    REQUIRE(!gm_approx(GMVec2D(1.5, 3.2), GMVec2D(1.5, 3.21)));

    REQUIRE(gm_is_on_segment(GMVec2D(2.0, 4.0), GMVec2D(1.0, 4.0), GMVec2D(7.0, 4.0)));
    REQUIRE(!gm_is_on_segment(GMVec2D(0.5, 4.0), GMVec2D(1.0, 4.0), GMVec2D(7.0, 4.0)));
    REQUIRE(!gm_is_on_segment(GMVec2D(8.0, 4.0), GMVec2D(1.0, 4.0), GMVec2D(7.0, 4.0)));
    REQUIRE(!gm_is_on_segment(GMVec2D(2.0, 3.0), GMVec2D(1.0, 4.0), GMVec2D(7.0, 4.0)));
    REQUIRE(gm_is_on_segment(GMVec2D(2.0, 2.0), GMVec2D(1.0, 1.0), GMVec2D(7.0, 7.0)));

    GMVec2D v1{0.0, 0.0};
    GMVec2D v2{0.0, 0.0};
    GMVec2D v3{0.0, 0.0};
    REQUIRE(gm_orientation(v1, v2, v3) == 0);

    v1 = {0.0, 5.0};
    v2 = {5.0, 4.0};
    v3 = {2.0, 2.0};
    REQUIRE(gm_orientation(v1, v2, v3) == 2);

    v1 = {5.0, 5.0};
    v2 = {1.0, 4.0};
    v3 = {2.0, 2.0};
    REQUIRE(gm_orientation(v1, v2, v3) == 1);

    REQUIRE(gm_between(0.5, 0.4, 0.6));
    REQUIRE(gm_between(1.1f32, 1.0f32, 1.2f32));
    REQUIRE(!gm_between(0.2, 0.4, 0.6));
    REQUIRE(!gm_between(0.8, 0.4, 0.6));

    GMLine l1{0.0, 0.0, 10.0, 0.0};
    v1 = {5.0, 1.0};
    v2 = gm_closest_point_on_segment(l1, v1);
    REQUIRE(gm_approx(v2.x, 5.0) && gm_approx(v2.y, 0.0));

    l1 = {1.0, 1.0, 8.0, 8.0};
    v1 = {3.0, 5.0};
    v2 = gm_closest_point_on_segment(l1, v1);
    REQUIRE(gm_approx(v2.x, 4.0) && gm_approx(v2.y, 4.0));
}

TEST_CASE("Test intersect line functions", "[math]" ) {
    GMLine l1{0.0, 0.0, 10.0, 0.0};
    GMVec2D v1{3.0, 0.0};
    REQUIRE(gm_intersect_line_point(l1, v1));

    v1 = {3.0, 1.0};
    REQUIRE(!gm_intersect_line_point(l1, v1));

    l1 = {1.0, 1.0, 6.0, 6.0};
    v1 = {3.0, 3.0};
    REQUIRE(gm_intersect_line_point(l1, v1));

    v1 = {3.0, 5.0};
    REQUIRE(!gm_intersect_line_point(l1, v1));

    l1 = {0.0, 0.0, 7.0, 0.0};
    GMLine l2{0.0, 1.0, 4.0, 1.0};
    REQUIRE(!gm_intersect_line_line1(l1, l2));

    l2 = {4.0, 2.0, 4.0, -2.0};
    REQUIRE(gm_intersect_line_line1(l1, l2));

    l2 = {0.0, 1.0, 4.0, 1.0};
    std::vector<GMVec2D> res = gm_intersect_line_line2(l1, l2);
    REQUIRE(res.size() == 0);

    l2 = {4.0, 2.0, 4.0, -2.0};
    res = gm_intersect_line_line2(l1, l2);
    REQUIRE(res.size() == 1);
    REQUIRE(gm_approx(res[0].x, 4.0) && gm_approx(res[0].y, 0.0));
}

TEST_CASE("Test intersect circle functions", "[math]" ) {
}

TEST_CASE("Test intersect rectangle functions", "[math]" ) {
}
