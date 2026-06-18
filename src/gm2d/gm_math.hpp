/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines math functions and classes.
*/

#ifndef FILE_GM_MATH_HPP_INCLUDED
#define FILE_GM_MATH_HPP_INCLUDED

// STD includes:
#include <cstdint>
#include <stdfloat>
#include <utility>
#include <array>
#include <optional>
#include <vector>

namespace gm2d {
enum struct GMAlignment: uint8_t {
    TOP_LEFT = 0,
    TOP_CENTER,
    TOP_RIGHT,

    MID_LEFT,
    MID_CENTER,
    MID_RIGHT,

    BTM_LEFT,
    BTM_CENTER,
    BTM_RIGHT
};

enum struct GMRepetition: uint8_t {
    FIXED = 0,
    FORWARD,
    BACKWARD,
    FORWARD_LOOP,
    BACKWARD_LOOP,
    PINGPONG_F,
    PINGPONG_B
};

const std::float32_t GM_EPSILON = 0.00001f32;

class GMVec2D {
    public:
        // Constructor:
        GMVec2D();
        GMVec2D(const std::float32_t);
        GMVec2D(const std::float32_t, const std::float32_t);
        GMVec2D(const GMVec2D &);
        GMVec2D(const std::pair<std::float32_t, std::float32_t> &);
        GMVec2D(const std::array<std::float32_t, 2> &);

        // Methods:
        void gm_add(const GMVec2D);
        void gm_mul(const std::float32_t);

        [[nodiscard]] std::float32_t gm_len1() const;
        [[nodiscard]] std::float32_t gm_len2() const;

        [[nodiscard]] GMVec2D gm_norm1() const;
        void gm_norm2();
        [[nodiscard]] std::float32_t gm_dist(const GMVec2D &) const;
        [[nodiscard]] std::float32_t gm_angle() const;
        [[nodiscard]] std::float32_t gm_cross(const GMVec2D &) const;

        [[nodiscard]] GMVec2D gm_rotate1(const std::float32_t) const;
        void gm_rotate2(std::float32_t);

        // Operators:
        void operator=(const GMVec2D);
        void operator+=(const GMVec2D);
        void operator-=(const GMVec2D);

        [[nodiscard]] GMVec2D operator*(const std::float32_t);
        void operator*=(const std::float32_t);

        friend bool operator==(const GMVec2D, const GMVec2D);
        friend bool operator!=(const GMVec2D, const GMVec2D);
        friend GMVec2D operator+(const GMVec2D, const GMVec2D);
        friend GMVec2D operator-(const GMVec2D, const GMVec2D);

        // Members:
        std::float32_t x;
        std::float32_t y;
};

class GMLine {
    public:
        // Constructor:
        GMLine();
        GMLine(const std::float32_t, const std::float32_t, const std::float32_t, const std::float32_t);
        GMLine(const GMVec2D &, const GMVec2D &);

        // Members:
        GMVec2D v1;
        GMVec2D v2;
};

class GMCircle {
    public:
        // Constructor:
        GMCircle();
        GMCircle(const std::float32_t, const std::float32_t, const std::float32_t);
        GMCircle(const GMVec2D &, const std::float32_t);

        // Methods:

        // Operators:

        // Members:
        GMVec2D ctr;
        std::float32_t r;
};

class GMRectangle {
    public:
        // Constructor:
        GMRectangle();
        GMRectangle(const std::float32_t, const std::float32_t, const std::float32_t, const std::float32_t);
        GMRectangle(const GMVec2D &, const GMVec2D &);

        // Methods:
        [[nodiscard]] std::float32_t gm_width();
        [[nodiscard]] std::float32_t gm_height();
        [[nodiscard]] GMVec2D gm_min_point() const;
        [[nodiscard]] GMVec2D gm_max_point() const;

        // Operators:

        // Members:
        GMVec2D v1;
        GMVec2D v2;
};


// Helper functions:
[[nodiscard]] bool gm_approx(std::float32_t, std::float32_t);
[[nodiscard]] bool gm_is_on_segment(const GMVec2D &, const GMVec2D &, const GMVec2D &);
[[nodiscard]] uint8_t gm_orientation(const GMVec2D &, const GMVec2D &, const GMVec2D &);
[[nodiscard]] bool gm_between(std::float32_t, std::float32_t, std::float32_t);

[[nodiscard]] bool gm_intersect_line_point(const GMLine &, const GMVec2D &);
[[nodiscard]] bool gm_intersect_line_line1(const GMLine &, const GMLine &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_line_line2(const GMLine &, const GMLine &);
[[nodiscard]] bool gm_intersect_circle_point(const GMCircle &, const GMVec2D &);
[[nodiscard]] bool gm_intersect_circle_line1(const GMCircle &, const GMLine &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_line2(const GMCircle &, const GMLine &);
[[nodiscard]] bool gm_intersect_circle_circle1(const GMCircle &, const GMCircle &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_circle2(const GMCircle &, const GMCircle &);
[[nodiscard]] bool gm_intersect_circle_rectangle1(const GMCircle &, const GMRectangle &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_rectangle2(const GMCircle &, const GMRectangle &);
[[nodiscard]] bool gm_intersect_rectangle_point(const GMRectangle &, const GMVec2D &);
[[nodiscard]] bool gm_intersect_rectangle_line1(const GMRectangle &, const GMLine &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_rectangle_line2(const GMRectangle &, const GMLine &);
[[nodiscard]] bool gm_intersect_rectangle_rectangle1(const GMRectangle &, const GMRectangle &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_rectangle_rectangle2(const GMRectangle &, const GMRectangle &);


}
#endif // FILE_GM_MATH_HPP_INCLUDED
